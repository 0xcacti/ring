# ring
Implementation of ping in rust. Not as good as the one with graphing.

I built this to learn ICMP.  This project will involve learning 
how to both construct and interpret different types of ICMP requests.  


Potential Improvements - 
I intended for this project to essentially be a better ping.  However, throughout 
the learning process I realized I gained most of the benefits of building the project 
without needing to optimize to achieve feature parity with existing ping implementations. 
Thus there are many potential improvements that could be made to this project.  
Specifically, you could achieve parity with the default ping coreutil, you could add custom level 
os configuration, add additional statistics collections, and even include some form of 
stats graphing.  You could of course, also have a number of code improvements, such 
as making the modules more library-like with cleaner APIs that aren't so tightly 
coupled to the way my CLI works.



