RBF_NAME='vfp8'

# Disable bridges
sudo sh -c 'echo 0 > /sys/class/fpga-bridge/fpga2hps/enable'
sudo sh -c 'echo 0 > /sys/class/fpga-bridge/hps2fpga/enable'
sudo sh -c 'echo 0 > /sys/class/fpga-bridge/lwhps2fpga/enable'

# Flash the FPGA
sudo dd if=$RBF_NAME.rbf of=/dev/fpga0

# Re-enable bridges
sudo sh -c 'echo 1 > /sys/class/fpga-bridge/lwhps2fpga/enable'
sudo sh -c 'echo 1 > /sys/class/fpga-bridge/hps2fpga/enable'
sudo sh -c 'echo 1 > /sys/class/fpga-bridge/fpga2hps/enable'