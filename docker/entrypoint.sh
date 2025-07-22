#!/bin/sh
# Default the TZ environment variable to UTC.
TZ=${TZ:-UTC}
export TZ

# Set environment variable that holds the Internal Docker IP.
INTERNAL_IP=$(ip route get 1 | awk '{print $(NF-2);exit}')
export INTERNAL_IP

# Switch to the container's working directory.
cd /app || exit 1

# Print Java version.
printf "\033[1m\033[33mgameserver@blockylabs~ \033[0mjava -version\n"
java -version

# Convert all occurrences of "{{VARIABLE}}" into "${VARIABLE}".
PARSED=$(echo "$STARTUP" | sed -e 's/{{/${/g' -e 's/}}/}/g')

# Copy all contents from /app to /home/container, including hidden files.
cp -r /app/. /home/container/

# Switch to the container's home directory.
cd /home/container || exit 1

# Adjust permissions on the container home directory.
#chmod -R 777 /home/container
# Note: The pattern /home/container/* will not include hidden files.
# If you need to change hidden files too, you may need additional logic.

printf "\033[1m\033[33mgameserver~ \033[0m"
echo "$PARSED"

# Execute the parsed startup command.
eval "$PARSED"
