#!/bin/sh
# Default the TZ environment variable to UTC.
TZ=${TZ:-UTC}
export TZ

# Set environment variable that holds the Internal Docker IP.
INTERNAL_IP=$(ip route get 1 | awk '{print $(NF-2);exit}')
export INTERNAL_IP
DEFAULT_STARTUP="java -Xms128M -Xmx2048M --add-modules=jdk.incubator.vector -XX:+UseG1GC -XX:+ParallelRefProcEnabled -XX:MaxGCPauseMillis=200 -XX:+UnlockExperimentalVMOptions -XX:+DisableExplicitGC -XX:+AlwaysPreTouch -XX:G1HeapWastePercent=5 -XX:G1MixedGCCountTarget=4 -XX:InitiatingHeapOccupancyPercent=15 -XX:G1MixedGCLiveThresholdPercent=90 -XX:G1RSetUpdatingPauseTimePercent=5 -XX:SurvivorRatio=32 -XX:+PerfDisableSharedMem -XX:MaxTenuringThreshold=1 -Dusing.aikars.flags=https:\\/\\/mcflags.emc.gs -Daikars.new.flags=true -XX:G1NewSizePercent=30 -XX:G1MaxNewSizePercent=40 -XX:G1HeapRegionSize=8M -XX:G1ReservePercent=20 -jar server.jar"
STARTUP=${STARTUP:-$DEFAULT_STARTUP}

# Switch to the container's working directory.
cd /app || exit 1

# Print Java version.
printf "\033[1m\033[33mdebug@dockercraft~ \033[0mjava -version\n"
java -version

PARSED=$(echo "$STARTUP" | sed -e 's/{{/${/g' -e 's/}}/}/g')

# Switch to the container's home directory.
cd /app || exit 1

# Adjust permissions on the container home directory.
chmod -R 777 /app
# Note: The pattern /home/container/* will not include hidden files.
# If you need to change hidden files too, you may need additional logic.

printf "\033[1m\033[33mgameserver~ \033[0m"
echo "$PARSED"

# Execute the parsed startup command.
eval "$PARSED"
