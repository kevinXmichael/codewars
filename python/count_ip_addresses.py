import ipaddress


def ips_between(start, end):
    # first_ips = list(map(int, start.split(".")))
    # second_ips = list(map(int, end.split(".")))
    # diff = 0
    # for first_ip, second_ip in zip(reversed(first_ips), reversed(second_ips)):
    #     # print(first_ip, second_ip)
    #     if bool(first_ip == 0) ^ bool(second_ip == 0):
    #         diff += abs(255 - first_ip - second_ip)
    #     else:
    #         diff += abs(first_ip - second_ip)
    # return diff
    ip1 = int(ipaddress.IPv4Address(str(start)))
    ip2 = int(ipaddress.IPv4Address(str(end)))
    return ip2 - ip1


print(ips_between("10.0.0.0", "10.0.0.50"), 50)
print(ips_between("20.0.0.10", "20.0.1.0"), 246)

# Implement a function that receives two IPv4 addresses, and returns the number of addresses between them (including the first one, excluding the last one).
#
# All inputs will be valid IPv4 addresses in the form of strings. The last address will always be greater than the first one.
#
# Examples
# ips_between("10.0.0.0", "10.0.0.50")  ==   50
# ips_between("10.0.0.0", "10.0.1.0")   ==  256
# ips_between("20.0.0.10", "20.0.1.0")  ==  246
