# bundle exec ruby -rinfor -e 'puts Infor::Sysinfo.new().total_memory()'
# bundle exec ruby -rinfor examples/sysinfo.rb

require 'infor'

# Initialize the Sysinfo object
sys = Infor::Sysinfo.new()

# First we update all information of our `System` struct.
sys.refresh_all()

puts("=> system:")
# RAM and swap information:
puts("total memory: #{sys.total_memory} bytes")
puts("used memory : #{sys.used_memory} bytes")
puts("total swap  : #{sys.total_swap} bytes")
puts("used swap   : #{sys.used_swap} bytes")

# Display system information:
puts("System name:             #{sys.name}")
puts("System kernel version:   #{sys.kernel_version}")
puts("System OS version:       #{sys.os_version}")
puts("System host name:        #{sys.host_name}")
