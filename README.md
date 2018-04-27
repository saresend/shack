# Shack 

[![Build Status](https://travis-ci.org/saresend/shack.svg?branch=master)](https://travis-ci.org/saresend/shack)


Shack, a dead simple CLI key value store, for saving simple stuff thats hard to remember. 


## Installation 

As of this writing, I haven't distributed binaries of shack, and so the only way to install it is via cargo, Rust's package manager! 

<code>cargo install shack</code>


## Usage 

[![asciicast](https://asciinema.org/a/cZHrSyoKerYPQeMeQFnNck1VO.png)](https://asciinema.org/a/cZHrSyoKerYPQeMeQFnNck1VO)



Shack will persist simple key values, and supports the Following commands: 

* *get* - will attempt to retrieve the value for a given key 
* *set* - will attempt to set the key value pair provided 
* *ls* - lists all the currently saved key value pairs 
* *del* - deletes the key value pair with the specified key


## Examples 

Suppose that you manage a couple servers, and each has their own IP address. When SSHing into them, instead of having to look up the IP address each time, you can use shack to save them like so: 

<code>shack set server1_ip 133.234.5.4</code>

<code>shack set server2_ip 132.133.54.1</code>

Then, when you want to retrieve them, you can simply list them: 

<code> shack ls </code>

or, you can get them by name 

<code> shack get server1_ip </code>






