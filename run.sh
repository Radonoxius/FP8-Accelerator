RBF_NAME='controller'

# Disable bridges
sudo sh -c 'echo 0 > /sys/class/fpga-bridge/fpga2hps/enable'
sudo sh -c 'echo 0 > /sys/class/fpga-bridge/hps2fpga/enable'
sudo sh -c 'echo 0 > /sys/class/fpga-bridge/lwhps2fpga/enable'

# Program FPGA
sudo dd if=$RBF_NAME.rbf of=/dev/fpga0

# Re-enable bridges ONLY if FPGA is ready
sudo sh -c 'echo 1 > /sys/class/fpga-bridge/lwhps2fpga/enable'
sudo sh -c 'echo 1 > /sys/class/fpga-bridge/hps2fpga/enable'
sudo sh -c 'echo 1 > /sys/class/fpga-bridge/fpga2hps/enable'

sudo ./test