# cube.js 特性分析

本文尝试记录我在试用 cube.js 过程中，对其的理解和分析。

# Data Modeling
我最感兴趣的是 cube.js 的模型建模能力。

数据集：[NorthWind](https://en.wikiversity.org/wiki/Database_Examples/Northwind/MySQL)
为了测试多事实支持能力，对该数据集做了简单扩展：
```sql
create table northwind.Orders
(
    OrderID    int auto_increment
        primary key,
    CustomerID int            null,
    EmployeeID int            null,
    OrderDate  datetime       null,
    ShipperID  int            null,
    freight    decimal(10, 2) null,   -- 新增运费字段
);

create table northwind.Inventories
(
    InventoryId int not null
        primary key,
    ProductId   int not null,
    quantity    int not null comment '库存数量'
);

```

1. Query: ( products.productname -> order_detail.count, orders.count, order_detail.quantity)
    ```sql
    SELECT
          `products`.`ProductName` `products__productname`, 
          count(`order_details`.OrderDetailId) `order_details__count`, 
          count(distinct `orders`.orderId) `orders__count`, 
          sum(`order_details`.`Quantity`) `order_details__quantity`
    FROM
          northwind.`OrderDetails` AS `order_details`
    LEFT JOIN northwind.`Orders` AS `orders` ON `order_details`.`OrderID` = `orders`.`OrderID`
    LEFT JOIN northwind.`Products` AS `products` ON `order_details`.`ProductID` = `products`.`ProductID`  
    GROUP BY 1 ORDER BY 2 DESC LIMIT 10000
    ```
    
    - 查询限制在单个 star-model 中
    - 对 star-model 中的非明细粒度的 count 会改写为 count(distinct) 符合预期。
2. Query: (product.productname -> order_details.quantity, orders.freight)
   ```sql 
   SELECT 
       q_0.`products__productname`, 
       `order_details__quantity` `order_details__quantity`, 
       `orders__freight` `orders__freight` 
   FROM (
        SELECT `main__products`.`ProductName` `products__productname`, sum(`main__order_details`.`Quantity`) `order_details__quantity` 
        FROM northwind.`OrderDetails` AS `main__order_details` 
        LEFT JOIN northwind.`Orders` AS `main__orders` ON `main__order_details`.`OrderID` = `main__orders`.`OrderID` 
        LEFT JOIN northwind.`Products` AS `main__products` ON `main__order_details`.`ProductID` = `main__products`.`ProductID`  GROUP BY 1
   ) as q_0 
   INNER JOIN (
        SELECT `keys`.`products__productname`, sum(`orders_key__orders`.freight) `orders__freight` 
        FROM (
            SELECT DISTINCT `orders_key__products`.`ProductName` `products__productname`, `orders_key__orders`.orderId `orders__order_id` 
            FROM northwind.`OrderDetails` AS `orders_key__order_details` 
            LEFT JOIN northwind.`Orders` AS `orders_key__orders` ON `orders_key__order_details`.`OrderID` = `orders_key__orders`.`OrderID` 
            LEFT JOIN northwind.`Products` AS `orders_key__products` ON `orders_key__order_details`.`ProductID` = `orders_key__products`.`ProductID` 
       ) AS `keys` 
       LEFT JOIN northwind.`Orders` AS `orders_key__orders` ON `keys`.`orders__order_id` = `orders_key__orders`.orderId
       GROUP BY 1
   ) as q_1 ON (q_0.`products__productname` = q_1.`products__productname` OR (q_0.`products__productname` IS NULL AND q_1.`products__productname` IS NULL)) 
   ORDER BY 2 DESC LIMIT 10000
   ```
   结论：
   1. cube.js 支持在同一个 star-model 中在不同层级上的度量（类似于多事实的概念），但生成的 SQL 不够优化，性能可能不佳。
   
3. Query: ( product.productname -> inventories.quantity, order_details.quantity)
   
   查询错误：`Can't find join path to join 'inventories', 'order_details', 'products'`
   结论：
   1. cube.js 查询中，限定只能使用单个查询图（限定一个 star-model ）,并不支持多事实的概念。
   2. 根据 query2：cube.js 支持在一个 star-model 中访问不同粒度的度量，能正确处理查询，但未能生成优化的 SQL。
   
4. YTD Query
   
   cube.js 支持的复杂计算相对较为有限，主要是 https://cube.dev/docs/product/data-modeling/concepts/multi-stage-calculations#period-to-date 文中介绍的：
   - Rolling Window， 包括 YTD, QTD, MTD 都是通过 rolling window 来实现的
   - Ranking 
   这里以一个简单的 YTD 为例来查看其执行过程：
```sql
SELECT q_0.`orders__orderdate_day`,
       `order_details__quantity`     `order_details__quantity`,
       `order_details__quantity_ytd` `order_details__quantity_ytd`
FROM (
   SELECT CAST(DATE_FORMAT(CONVERT_TZ(`main__orders`.`OrderDate`, @@session.time_zone, '+00:00'),
                              '%Y-%m-%dT00:00:00.000') AS DATETIME) `orders__orderdate_day`,
             sum(`main__order_details`.`Quantity`)                  `order_details__quantity`
   FROM northwind.`OrderDetails` AS `main__order_details`
   LEFT JOIN northwind.`Orders` AS `main__orders`
                         ON `main__order_details`.`OrderID` = `main__orders`.`OrderID`
   WHERE (`main__orders`.`OrderDate` >=
             TIMESTAMP(convert_tz('1996-07-01T00:00:00.000', '+00:00', @@session.time_zone)) AND
             `main__orders`.`OrderDate` <=
             TIMESTAMP(convert_tz('1996-12-31T23:59:59.999', '+00:00', @@session.time_zone)))
   GROUP BY 1
) as q_0
INNER JOIN (
   SELECT `orders.orderdate_series`.`date_from` `orders__orderdate_day`,
         sum(`order_details__quantity_ytd`)    `order_details__quantity_ytd`
   FROM (
      SELECT TIMESTAMP(dates.f) date_from, TIMESTAMP(dates.t) date_to
      FROM (
            select '1996-07-01T00:00:00.000' f, '1996-07-01T23:59:59.999' t
               UNION ALL
               ... -- 这里省略掉几百行类似的 SQL 代码
               UNION ALL
               select '1996-12-31T00:00:00.000' f, '1996-12-31T23:59:59.999' t
      ) AS dates
   ) AS `orders.orderdate_series`
   LEFT JOIN (
      SELECT CAST(DATE_FORMAT(CONVERT_TZ(`order_details_quantity_ytd_cumulative__orders`.`OrderDate`,@@session.time_zone, '+00:00'), '%Y-%m-%dT00:00:00.000') 
                  AS DATETIME) `orders__orderdate_day`,
             sum(`order_details_quantity_ytd_cumulative__order_details`.Quantity) `order_details__quantity_ytd`
     FROM northwind.`OrderDetails` AS `order_details_quantity_ytd_cumulative__order_details`
     LEFT JOIN northwind.`Orders` AS `order_details_quantity_ytd_cumulative__orders`
                                          ON `order_details_quantity_ytd_cumulative__order_details`.`OrderID` =
                                             `order_details_quantity_ytd_cumulative__orders`.`OrderID`
     WHERE (CONVERT_TZ(`order_details_quantity_ytd_cumulative__orders`.`OrderDate`,@@session.time_zone, '+00:00') >=
            CAST(DATE_FORMAT(TIMESTAMP('1996-07-01T00:00:00.000'), '%Y-01-01T00:00:00.000') AS DATETIME) AND
            CONVERT_TZ(`order_details_quantity_ytd_cumulative__orders`.`OrderDate`,@@session.time_zone, '+00:00') <= TIMESTAMP('1996-12-31T23:59:59.999'))
     GROUP BY 1
   ) AS `order_details_quantity_ytd_cumulative__base`
     ON （`order_details_quantity_ytd_cumulative__base`.`orders__orderdate_day` >= 
          CAST(DATE_FORMAT(`orders.orderdate_series`.`date_from`, '%Y-01-01T00:00:00.000') AS DATETIME) AND
          `order_details_quantity_ytd_cumulative__base`.`orders__orderdate_day` <= `orders.orderdate_series`.`date_to`）
   GROUP BY 1
) as q_1 ON (q_0.`orders__orderdate_day` = q_1.`orders__orderdate_day` OR
                                            (q_0.`orders__orderdate_day` IS NULL AND
                                             q_1.`orders__orderdate_day` IS NULL))
ORDER BY 1 ASC
LIMIT 10000
```
   分析结论：
   1. cube.js 对计算度量的额表达能力相比 MDX 来说非常有限，其更接近于 Smartbi 支持的快速计算，都是内置支持的，不太具备了良好的扩展能力。
   2. 在 YTD 计算时，目前的实现方式是通过 conditional join 来实现的，其效果与我们的基于窗口函数的实现相似，但执行效率估计会低一些
   3. 目前进行 YTD 计算时，对查询条件有一定的限制，必须指定 开始日期、结束日期。
   4. CubeJS 有一个 试验性的 Tesseract 引擎（估计与 powerbi 的 VertiPaq 类似）。

总体来看：CubeJS 的建模能力相比 PowerBI, SmartBi 目前的建模能力，要弱很多。