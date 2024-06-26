# ring
Implementation of ping in rust. Not as good as the one with graphing

I built this to learn ICMP.  This project will involve learning 
how to both construct and interpret different types of ICMP requests.  
I aim to have full feature parity with the default ping, including all 
flags.


TODO: 


- [ ] Finish refactor to have IPV4 and IPV6 types

- [ ] Add ipv6 support
    - [ ] Add ipv6 header generation and serialization
    - [ ] Add ipv6 icmp header generation and serialization
    - [ ] Add socket support for ipv6 

- [ ] Echo resoponse parsing 
    - [ ] Create echo response type 
    - [ ] Deserialize echo response into type
    
- [ ] Add payloads
- [ ] Formating 
    - [ ] Decide what should be included in the output 
    - [ ] Add coloration 
    - [ ] See if you can add coloration to clap
- [ ] Collect ping statistics
- [ ] Make packet generation more customizable - sequence number etc
- [ ] Add encapsulation
