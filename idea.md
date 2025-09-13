1. Build a jq clone

## Features devided in phase
### A
1. a basic parser with all types and everything that can check if the given json is syntactically correct or not
2. support only 1 command '.'

### B
1. allow basic filtering feats
Object Identifier-Index: .foo, .foo.bar
       The simplest useful filter is .foo. When given a JSON object (aka dictionary or hash) as input, it produces the value at the key "foo", or null  if
       there's none present.

       A filter of the form .foo.bar is equivalent to .foo|.bar.

       This  syntax  only  works for simple, identifier-like keys, that is, keys that are all made of alphanumeric characters and underscore, and which do
       not start with a digit.

       If the key contains special characters, you need to surround it with double quotes like this: ."foo$", or else .["foo$"].

       For example .["foo::bar"] and .["foo.bar"] work while .foo::bar does not, and .foo.bar means .["foo"].["bar"].

           jq '.foo'
              {"foo": 42, "bar": "less interesting data"}
           => 42

   Optional Object Identifier-Index: .foo?
       Just like .foo, but does not output even an error when . is not an array or an object.

           jq '.foo?'
              {"foo": 42, "bar": "less interesting data"}
           => 42

           jq '.foo?'
              {"notfoo": true, "alsonotfoo": false}
           => null

           jq '.["foo"]?'
              {"foo": 42}
           => 42

           jq '[.foo?]'
              [1,2]
           => []



for later feature just follow next example in man db