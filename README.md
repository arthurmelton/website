# Website

This is a static website generator for my website at 
[arthurmelton.com](https://arthurmelton.com). To use this make a folder caled
`blogs`, `pages`, `static`, a file called `template.html.hbs`, and a file called
`config.toml`. The blogs folder is to hold mardown files of all your blogs with a
little bit of toml at the top. Pages can have sub directories and each .md page
will be fully converted into a .html page at the same relitive path. Static is 
for all your file that you dont want to change (css, images, etc). 
The `template.html.hbs` is a handlebars file with all of your configuration 
needs. To see an example look in the `example` folder. `config.toml` is a file
for the configuration of your site, look in the `example` folder for a example.
