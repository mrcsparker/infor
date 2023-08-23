# Infor

Ruby wrapper for [Sysinfo](https://github.com/GuillaumeGomez/sysinfo)

## Installation

Install the gem and add to the application's Gemfile by executing:

    $ bundle add infor

If bundler is not being used to manage dependencies, install the gem by executing:

    $ gem install infor

## Usage

```ruby
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
```

## Development

After checking out the repo, run `bin/setup` to install dependencies. Then, run `rake spec` to run the tests. You can also run `bin/console` for an interactive prompt that will allow you to experiment.

To install this gem onto your local machine, run `bundle exec rake install`. To release a new version, update the version number in `version.rb`, and then run `bundle exec rake release`, which will create a git tag for the version, push git commits and the created tag, and push the `.gem` file to [rubygems.org](https://rubygems.org).

## Compiling

```sh
> bundle install
> bundle exec rake compile
```

## Contributing

Bug reports and pull requests are welcome on GitHub at [https://github.com/mrcsparker/infor](https://github.com/mrcsparker/infor).

## License

The gem is available as open source under the terms of the [MIT License](https://opensource.org/licenses/MIT).
