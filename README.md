When the connect losts, it shuts down.

Available for Windows and Unix-like systems(and MacOS)

Use **cargo build --release** to build.


Usage: ping_shutdown [OPTIONS]

Options:
  -i, --ip <IP>
          the ip address or website you want to check [default: bing.com]

  -n, --secs-for-normal-loop <SECS_FOR_NORMAL_LOOP>
          time between two normal check [default: 60]
          
  -e, --secs-for-emergency-loop <SECS_FOR_EMERGENCY_LOOP>
          time between two emegency check [default: 20]
          
  -t, --times-for-emergency-loop <TIMES_FOR_EMERGENCY_LOOP>
          times for emergency loop [default: 3]
          
  -h, --help
          Print help
          
  -V, --version
          Print version


**Please please ignore my poor English(⋟﹏⋞)**

Have a nice day. ヾ(✿ﾟ▽ﾟ)ノ