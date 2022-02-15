import re


def domain_name(url):
    pattern = "^(?:(http(s)?)?:\/\/)?(?:[^@\/\n]+@)?(?:www\.)?([^:\/?\n]+)\."
    m = re.search(pattern, url)
    return m.group(len(m.groups())).split(".")[0]


domain_name("http://github.com/carbonfive/raygun")

# Write a function that when given a URL as a string, parses out just the domain name and returns it as a string. For example:
# domain_name("http://github.com/carbonfive/raygun") == "github"
# domain_name("http://www.zombie-bites.com") == "zombie-bites"
# domain_name("https://www.cnet.com") == "cnet"
