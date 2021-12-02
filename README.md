# Ring Hook
Android ART Hook工具

👴的毕设,打算从native层到java层的hook都搞一遍，最终目标是能支持hook用户apk的东西

# TODO LIST

下面这些东西按顺序实现。除0号外其中每一个步骤都包换对同类项目的调研、学习内容。

0. 氪200~500块买个纯净的google aosp测试机先，优先考虑pixel或者nexus系列

1. 搭建框架，搞定dex文件注入问题，设计好各个功能模块之间的聚合与耦合
   
2. 实现面向系统的 native hookers
   
3. 实现面向系统的 java hookers

4. 继续研究aosp源码中关于classloader的部分，找到可以实现hook 用户java代码的方法

5. 研究linux linker机制，找到可以实现hook用户native符号的方法