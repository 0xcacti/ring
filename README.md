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
    - [ ] Set customization options

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
- [ ] IPV4 TTL
        :: Will allow - need to decide how to handle when not IPV6 probably just ignore
- [ ] IPV6 hop limit 
        :: Will allow - need to decide how to handle when not IPV6 probably just ignore
- [ ] ICMP header ID 
        :: Will allow - if not specified will be the process id
- [ ] ICMP header sequence number ( maybe )
        :: Will allow - if not specified will the the number of packets sent
- [ ] Payload
    - [ ] Can have payload include random bytes 
    - [ ] Can have payload include a sequence number 
    - [ ] Can have payload include a timestamp

ping flags - 

- [ ] -a audible ping
- [ ] -c count - number of pings
- [ ] -i interval - time to wait between pings 
- [ ] -t ttl - set ipv4 ttl 
- [ ] -h hop limit - set ipv6 hop limit
- [ ] -T timeout - time to wait for a response



