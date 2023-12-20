# -*- mode: ruby -*-
# vi: set ft=ruby :

Vagrant.configure("2") do |config|
  config.vm.box = "generic-x64/fedora39"
  config.vm.box_version = "4.3.8"

  config.vm.box_check_update = false

  config.ssh.forward_agent = true
  config.ssh.forward_x11 = true

  config.vm.network "forwarded_port", guest: 5900, host: 5900
  config.vm.network "forwarded_port", guest: 8080, host: 8080
  config.vm.network "forwarded_port", guest: 5901, host: 5901
  config.vm.network "forwarded_port", guest: 5173, host: 5173
  # config.vm.network "forwarded_port", guest: 80, host: 8080, host_ip: "127.0.0.1"

  # config.vm.network "private_network", ip: "192.168.33.10"

  # config.vm.network "public_network"

  # config.vm.synced_folder "../data", "/vagrant_data"

  # config.vm.synced_folder ".", "/vagrant", disabled: false
  config.vm.synced_folder ".", "/home/vagrant/VroomCloud", type: "nfs", nfs_version: 4


  config.vm.provider :libvirt do |v|
    v.cpus = 4
    v.memory = 8192
    v.storage :file, :size => '30G'
  end


  # config.vm.provision "shell", inline: <<-SHELL
  #   apt-get update
  #   apt-get install -y apache2
  # SHELL
end
