# Change Log

All notable changes to this project will be documented in this file

# v.0.4.0
## (11-11-2023)
* Update to embedded-graphics 0.8 and remove nodejs dependency [ @spookyvision ]

# v.0.3.0
## (10-01-2021)

* instead of parsing CSS for every pixel color, draw directly into a persistent RGBA array. Also make copying to the actual canvas an explicit flush() operation. [ @spookyvision ] 
* fix: differentiate between effective canvas dimension (includes scale+spacing) and simulated ("original") size [ @spookyvision ] 
* fix: animation example compiles again (type mismatch) [ @spookyvision ] 
* cleanup (variable names, unused imports, error messages) [ @spookyvision ] 
* prettier animated example [ @spookyvision ] 

# v0.2.1
## (21-11-2021)

* Fix readme link for simulator examples [ @jacobrosenthal ]
* Fix other links pointing to original embedded-graphics repo(https://github.com/jamwaffles/embedded-graphics)[ @rahul-thakoor ]

# v0.2.0
## (21-11-2021)

* improve fill/clear performance by 1000x or so [ @spookyvision ]
* add optional parent node [ @spookyvision ]
* add animated example (adapted from wasm-bindgen) [ @spookyvision ]
* update examples' dependencies [ @spookyvision ] 