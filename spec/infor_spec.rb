# frozen_string_literal: true

RSpec.describe Infor do
  it "has a version number" do
    expect(Infor::VERSION).not_to be nil
  end

  it "have cpus" do
    sys = Infor::Sysinfo.new
    expect(sys.cpus.size).to be > 0
  end

  it "have components" do
    sys = Infor::Sysinfo.new
    expect(sys.components.size).to be > 0
  end

  it "have users" do
    sys = Infor::Sysinfo.new
    expect(sys.users.size).to be > 0
  end

  it "have disks" do
    sys = Infor::Sysinfo.new
    expect(sys.disks.size).to be > 0
  end

  it "have networks" do
    sys = Infor::Sysinfo.new
    expect(sys.networks.size).to be > 0
  end
end
