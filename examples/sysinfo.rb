# bundle exec ruby -rinfor -e 'puts Infor::Sysinfo.new().total_memory()'
# bundle exec ruby -rinfor examples/sysinfo.rb

require 'infor'

# Initialize the Sysinfo object
sys = Infor::Sysinfo.new()

# First we update all information of our `System` struct.
sys.refresh_all()

# We display all disks' information:
puts("\n=> disks:")
sys.disks.each do |disk|
    puts disk.to_hash
end

# Network interfaces name, data received and data transmitted:
puts("\n=> networks:")
sys.networks.each do |network|
    puts("#{network.interface}: #{network.received}/#{network.transmitted} B")
end

# Components temperature:
puts("\n=> components:")
sys.components.each do |component|
    puts component.to_hash
end

puts("\n=> system:")
# RAM and swap information:
puts("total memory: #{sys.total_memory} bytes")
puts("used memory : #{sys.used_memory} bytes")
puts("total swap  : #{sys.total_swap} bytes")
puts("used swap   : #{sys.used_swap} bytes")

puts("\n")

# Display system information:
puts("System name:             #{sys.name}")
puts("System kernel version:   #{sys.kernel_version}")
puts("System OS version:       #{sys.os_version}")
puts("System host name:        #{sys.host_name}")

# Number of CPUs:
puts("\nNB CPUs: #{sys.cpus.size}")
