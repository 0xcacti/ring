# ring
Implementation of ping in rust. Not as good as the one with graphing

I built this to learn ICMP.  This project will involve learning 
how to both construct and interpret different types of ICMP requests.  
I aim to have full feature parity with the default ping, including all 
flags.


TODO: 


- [x] Finish refactor to have IPV4 and IPV6 types
- [ ] Add ipv6 support
    - [x] Add ipv6 header generation and serialization
    - [x] Add ipv6 icmp header generation and serialization
    - [x] Add socket support for ipv6 
    - [ ] Test it works on linux with headers included

- [x] Add domain resoleution
- [x] Echo resoponse parsing 
    - [x] Create echo response type 
    - [x] Deserialize echo response into type

- [ ] Make packet generation more customizable 
    - [x] Set customization options in clap
    - [x] Implement defaults 
    - [x] Use the clap args in the actual packet

- [ ] Compute lengths at runtime
- [ ] Add payloads

- [ ] Formating 
    - [ ] Decide what should be included in the output 
    - [ ] Add coloration 
    - [ ] See if you can add coloration to clap

- [ ] Collect ping statistics
- [ ] Add the rest of pings features
- [ ] Add encapsulation
- [ ] Clean up errors
- [ ] Clean up print statements



On customization - 

Here is everything that could be customized
- [ ] Payload
        :: not going to have customization, will just be 32 random bytes - room for improvements

ping flags - 

- [ ] -a audible ping
- [ ] -c count - number of pings
- [ ] -i interval - time to wait between pings 
- [ ] -t ttl - set ipv4 ttl 
- [ ] -h hop limit - set ipv6 hop limit
- [ ] -T timeout - time to wait for a response


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
