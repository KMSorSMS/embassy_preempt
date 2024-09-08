start

define dss
  dashboard source -output /dev/pts/$arg0
  dashboard source -style height 0
end

define dsa
  dashboard assembly -output /dev/pts/$arg0
  dashboard assembly -style height 0
end

define ash
  dashboard assembly -style height $arg0
end
define ssh
  dashboard source -style height $arg0
end

define sbp
  save breakpoints bp.gdb
end