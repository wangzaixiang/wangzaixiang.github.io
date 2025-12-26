# 图形语法 (The Grammar of Graphics)

**图形语法 (The Grammar of Graphics)** 是一套由 Leland Wilkinson 在 1999 年提出的理论体系，它彻底改变了我们描述和构建统计图形的方式。

在 GoG 出现之前，可视化通常是基于**图表类型 (Chart Typology)** 的：你选择一个“饼图”或“柱状图”，然后填充数据。这就好比学习语言时依靠“短语手册”（比如死记硬背“厕所在哪里？”）。

GoG 则提供了**语法 (Grammar)**：它定义了一组基本组件和规则，让你能够通过组合这些组件来构建任何可能的图形。这就好比学习了主谓宾结构，你可以创造出从未见过的句子。

## 核心层级模型 (The Layered Model)

Wilkinson 的理论将一个图形拆解为多个独立的步骤或层级。一个图形的生成过程，就是数据流经这些层级的过程：

1.  **Data (数据)**:
    *   从原始数据集开始。
    *   操作：Ingest, Filter.
2.  **Trans (Transformation/Varset - 变换)**:
    *   对变量进行数学变换或统计计算。
    *   例子：`rank`, `log`, `binning` (分箱), `summary` (求和/平均)。
    *   *在 Vega 中对应*: `transforms`。
3.  **Scale (比例尺)**:
    *   这是最关键的概念之一。它负责将**数据空间 (Data Domain)** 映射到 **视觉空间 (Visual Range)**。
    *   例子：将 `0-100` (数值) 映射到 `0px-500px` (位置)，或者映射到 `蓝色-红色` (颜色)。
    *   *在 Vega 中对应*: `scales`。
4.  **Coord (Coordinates - 坐标系)**:
    *   定义图形所在的几何空间。
    *   例子：笛卡尔坐标系 (Cartesian)、极坐标系 (Polar/Pie)、对数坐标系。
    *   *在 Vega 中对应*: 通常隐含在 `scales` 和 `axes` 的组合中，或者通过 `projections` (地理投影) 体现。
5.  **Element (Graph/Geom - 图形元素)**:
    *   实际代表数据的几何形状。
    *   例子：点 (Point)、线 (Line)、矩形 (Rect/Bar)、多边形 (Area)。
    *   这些元素具有**Aesthetic Attributes (视觉属性)**，如位置 (x, y)、颜色 (color)、大小 (size)、形状 (shape)。
    *   *在 Vega 中对应*: `marks`。
6.  **Guide (辅助元素)**:
    *   帮助用户解读图形的辅助设施，本质上是 Scale 的逆向可视化。
    *   例子：坐标轴 (Axes - 对应位置比例尺)、图例 (Legends - 对应颜色/形状比例尺)。
    *   *在 Vega 中对应*: `axes` 和 `legends`。

## 举例：解构散点图 (Scatter Plot)

使用图形语法的视角，我们不叫它“散点图”，而是描述为：

1.  **Data**: 表格数据，包含变量 A 和 B。
2.  **Scale**:
    *   Scale X: 将变量 A 的范围映射到宽度 [0, 500]。
    *   Scale Y: 将变量 B 的范围映射到高度 [500, 0] (屏幕坐标通常 y 轴向下)。
3.  **Coord**: 笛卡尔坐标系。
4.  **Element**: 使用 **Point (点)** 几何体。
    *   点的 `x` 属性绑定到 Scale X(A)。
    *   点的 `y` 属性绑定到 Scale Y(B)。
5.  **Guide**: 添加 X 轴和 Y 轴来显示刻度。

## 为什么这很重要？

1.  **正交性 (Orthogonality)**: 你可以独立修改任何一层而不破坏其他层。
    *   想把散点图变成气泡图？只需修改 **Element** 的 `size` 属性绑定。
    *   想把柱状图变成玫瑰图 (Coxcomb)？只需将 **Coord** 从笛卡尔坐标改为极坐标。
2.  **表达力 (Expressiveness)**: 你不再受限于软件预设的“图表库”。你可以组合出复杂的混合图表（例如：在地图上叠加饼图，而饼图的大小由第三个变量控制）。

## 与 Vega 的联系

Vega 是 GoG 理论在 Web 环境下的**一种**实现，但它做了关键的扩展：

*   **GoG (Original)**: 主要是**静态**的描述。一旦渲染完成，过程结束。
*   **Vega**: 引入了 **Signal (信号)** 和 **Event (事件)**。它让上述的每一个步骤（特别是 Scale 和 Transformation）都变成了**参数化**的。
    *   如果 Scale 的 `domain` 依赖于一个 Signal？那么当 Signal 变动时，Scale 更新 -> Element 位置更新 -> 图形重绘。
    *   这就是 Vega 所谓的 **"Interactive Grammar of Graphics"**。
