=# typeup test-drive

# formatting

==bold== and //italic//, the easy-to-type way, and
*bold* and _italic_, the nice-to-read way. All of these
line breaks
are ignored and this is
one paragraph

_[
hello!
each line here is its own <p>
]

[[a link: https://skuz.xyz]]

| A quote - it's just words! -- An author

## media

media are customizable. 'image' and 'video' are built-in.

image[[alt text goes here: https://skuz.xyz/favicon.png]]

# lists

[
lists are easy to type, no line prefixes needed
one item goes on each line. Bold/italic etc. are processed
To add sub-lists, just nest lists
this is an ordered list:
{
eat
sleep
repeat
}
]

# h1
## h2
### h3
#### h4
##### h5
###### h6

# thematic breaks

theme 1
---
theme .

# code

'code' is cool. You can also use markdown-style '`', which is more code-friendly and shorter but a bit harder to type.

'''
plain multiline code snippets also work
'''

```
3 backticks do the same thing
```

## tables

In tables, you must prefix the header row with your separator character. You can use anything

#{
,name, statically typed, GC, compiled
Go, yes, yes, yes
Zig, yes, no, yes
Python, no, yes, no
}

## customization

typeup can be customized with custom blocks. This block:

''
go:{
package main
}
''

will get passed to the function go, which will syntax-highlight the content and render it to HTML. The possibilities are endless. The 'go-typeup' renderer can use native Go functions and command-line programs as custom blocks.

go:{
package main

// For syntax highlighing, custom blocks can be used.
func main(){}
}

## metadata

you can specify metadata in a ''@{}'' block, as key-value pairs of strings. These can be used by static site generators, Wikis and other programs

Additionaly, the shortcut ''=#'' exists which creates a `h1` element with its argument ==and== sets the ''title'' attribute to the content