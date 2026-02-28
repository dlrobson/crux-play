# 1. Update and install dependencies
sudo apt-get update
sudo apt-get install -y ca-certificates curl gnupg

# 2. Create the keyring directory
sudo mkdir -p /etc/apt/keyrings

# 3. Download and add the NodeSource GPG key (example for Node 20)
curl -fsSL https://deb.nodesource.com/gpgkey/nodesource-repo.gpg.key | sudo gpg --dearmor -o /etc/apt/keyrings/nodesource.gpg

# 4. Create the repository entry
NODE_MAJOR=20
echo "deb [signed-by=/etc/apt/keyrings/nodesource.gpg] https://deb.nodesource.com/node_$NODE_MAJOR.x nodistro main" | sudo tee /etc/apt/sources.list.d/nodesource.list

# 5. Update and install
sudo apt-get update
sudo apt-get install nodejs -y