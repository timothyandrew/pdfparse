> Since I didn’t manage to re-mount the root partition as read-only to avoid further damage softly, I used the big hammer to remount everything read-only immediately: # echo u > /proc/sysrq-trigger

> I knew the inode number of the deleted file! As I mentioned before, my understanding of file systems was (and is) rather naive, and I was pretty optimistic to be able to recover the file using that information. Isn’t that sort of what a journaling file

> system is for? Recovering the file this way hover appeared to be impossible. ext4magic and extundelete are powerful tools that did find some deleted files on my disk – but not the one I was looking for, even after trying different options for over two hours

> There is this one other approach to file recovery that is often recommended on the internet, usually for “small text files”: Just grep your whole disk for known parts of its contents! So why wouldn’t this work on larger non-text files as well?

> all FLV files that contain video start with the byte sequence FLV\x01\x05 . So let’s search our 2 TB disk for that byte sequence and print out the byte offset of all occurences!

> This took roughly 7 hours.

> In total, the search found 126 FLV file headers on our disk. This was pretty reassuring, since we had 122 FLV files still known to the file system – so there are at least four FLV byte sequences without a filename!

> Now, all that was left to do was writing out the byte sequences of (at least) 1.6 GB starting at the five possible byte offsets. Just to be safe, I exported 1.8 GB of each:

> I then downloaded the five files, and indeed, the one with the highest position on disk contained the video file I accidentally deleted. Except some very minor corruption of less than a second somewhere in the video, the video was fully recovered. Phew.

This sounds interesting. Why were the file *contents* corrupted?