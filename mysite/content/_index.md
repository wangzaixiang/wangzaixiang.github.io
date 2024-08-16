+++
title = "程序人生"


# The homepage contents
[extra]
lead = '本站基于 zola + AdiDoks 主题搭建，主要用于记录个人学习笔记。'
#url = "/docs/getting-started/introduction/"
#url_button = "Get started"
#repo_version = "GitHub v0.1.0"
#repo_license = "Open-source MIT License."
#repo_url = "https://github.com/aaranxu/adidoks"

# Menu items
[[extra.menu.main]]
name = "Projects"
section = "docs"
url = "/docs/getting-started/introduction/"
weight = 10

[[extra.menu.main]]
name = "Blog"
section = "blog"
url = "/blog/"
weight = 20

[[extra.menu.main]]
name = "Monthly"
section = "monthly"
url = "/monthly/"
weight = 30

[[extra.list]]
title = "wsql"
content = """<a href='https://github.com/wangzaixiang/wsql'>wsql</a> is a diirect style Scala3 API for sql processing. 
provide mapping for case class via macro, batch insert support, and more.
"""

[[extra.list]]
title = "wjson"
content = """<a href="https://github.com/wangzaixiang/wjson/">wjson</a> is a direct style Scala3 API for JSON processing.
provide ADT to JSON mapping, JSON/JSON5 parsing, JSON schema generator, and a powerful JSON pattern matching DSL. 
"""

[[extra.list]]
title = "dapeng-soa"
content = """<a href="https://github.com/dapeng-soa/dapeng-soa">dapeng-soa</a> is a high-performance, scalable microservice framework.
Rich features, including service governance, service discovery, service routing, service monitoring, API sites, API test tooling, etc.
support Java, Scala.
"""

#[[extra.list]]
#title = "Full text search"
#content = "Search your Doks site with FlexSearch. Easily customize index settings and search options to your liking."
#
#[[extra.list]]
#title = "Page layouts"
#content = "Build pages with a landing page, blog, or documentation layout. Add custom sections and components to suit your needs."
#
#[[extra.list]]
#title = "Dark mode"
#content = "Switch to a low-light UI with the click of a button. Change colors with variables to match your branding."

+++
