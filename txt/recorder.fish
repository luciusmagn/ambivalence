#!/usr/bin/fish
set old_date (date | tr " " "_")

while true
	ffmpeg -i 'http://192.168.25.1:8080/?action=stream' -fs $argv[1] "video"$old_date".h264"
	ffmpeg -i "video"$old_date".h264" "video"$old_date".mkv" &
	rm -rf "video"$old_date".h264"
	set old_date (date | tr " " "_")
	echo "next!"
	if [ (df | grep -E '/$' | awk '{print $5}' | tr -d "%") -gt 85 ]
		rm -rf (ls -1 | sort | grep video | head -n 1)
	end
end