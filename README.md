This is Caleb Grep
Crep

2/11/26 2:00pm
I want to start by making it just search through a single file super quick
This can be accomplished by looking at the file path
If the file path /path/to/file.txt
we can then use fs::metadata to get the information about it.

2/11/26 7:51pm
Ok I did line major searching, but I want to use the boyer moore algorithm that is actually used in for real Grep
before I do that though, I want to make the row major not just print matches, but return them as a list of "match" structs.
So now I'll do this struct business.

2/11/26 8:37
Ok I though about it and making a struct and saving results will really bloat this thing like crazy
Large files would get these crazy long vectors and this would not be good....
Instead I will just make it print and forget using a "print_match" function.

2/11/26 9:00
OK I removed struct and just have it print. It works but doesn't work for Regex yet. I think making it match regex on line major is next goal
After that, I will research the boyer moore algorithm and implement that

2/12/26 3:44pm
I think it would be cool to do a limit on the recursive call for files.

I am going to see about Regex pattern matching on single files now. then...
I am going to do folder searching and it will just do a single folder.
Flags will be integrated later.
I want to do a callback for filesearching within the folder search

2/12/26 4:17
Ok basic REGEX works, "cargo run "(?i)rust" .\tests\test_input1.txt" worked which was really cool
It is printing separately in each line, which I would prefer it do each match on one line
next goal is to do that,

2/16/10:03
Honestly on reflection I think the printing all matches on one line will either:

1. Destroy the constant space aspect of the program by requiring a a vector of matches for each line
2. Requiring printing out a character at a time which would suck

EIther way, this will complicate the actual use of the tool, matches on different lines should be treated differently
Instead I am now going to wire up flags
folder searching w/ line major callback -> flags -> boyer moore.

Ok so I have to make a decision if I want to use BFS or DFS when seraching the folders
It may make sense to use DFS because it is less memory, but idk
I think I'll have a flag that allows BFS.
I also think I need to restructure it to divide into folder, file, symlink within one function instead of main.

2/18
I have folders working, but now I need to add the flags to modify behavior. Order will be pretty loose.
I fixed the parsing arguments now to handle flags

2/19
Major flags to consider:
single char flags
-r = recursive searching
-c = just print count instead of matches
-n = use line major as opposed to boyer moore
--recurse-depth = how far in the directory to go down

I am going to build an options struct with all possible options. Input gets parsed and defaults are set unless the flags change it.
Then, the main loop will pass the options struct everywhere, and it will be consulted inside inner functions, NOT the main loop.
