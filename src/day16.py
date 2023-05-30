import sys
import math

def seek_bits(bits, length):
    return bits[length:], int(bits[:length], 2), bits[:length]

def parse_packet(bits):
    bits, version, _ = seek_bits(bits, 3)
    bits, packet_type, _ = seek_bits(bits, 3)

    if packet_type == 4:
        nibbles = []
        while True:
            bits, next_flag, _ = seek_bits(bits, 1)
            if next_flag == 0:
                break
            bits, _, next_nibble = seek_bits(bits, 4)
            nibbles.append(next_nibble)
        bits, _, next_nibble = seek_bits(bits, 4)
        nibbles.append(next_nibble)
        value = int(''.join(nibbles), 2)
            
        return {
            'version': version,
            'type': packet_type,
            'value': value
        }, bits
    else:
        bits, length_type, _ = seek_bits(bits, 1)
        if length_type == 0:
            bits, total_length, _ = seek_bits(bits, 15)
            remainding_length = len(bits)

            subpackets = []
            while remainding_length - len(bits) < total_length:
                parsed, bits = parse_packet(bits)
                subpackets.append(parsed)

            return {
                'version': version,
                'type': packet_type,
                'subpackets': subpackets
            }, bits
        else:
            bits, subpacket_count, _ = seek_bits(bits, 11)

            subpackets = []
            for _ in range(subpacket_count):
                parsed, bits = parse_packet(bits)
                subpackets.append(parsed)

            return {
                'version': version,
                'type': packet_type,
                'subpackets': subpackets
            }, bits

def sum_versions(packet):
    if 'subpackets' in packet:
        return packet['version'] + sum(sum_versions(subpacket) for subpacket in packet['subpackets'])
    else:
        return packet['version']

def solve1(file_data):
    binary_data = ''.join('{0:b}'.format(int(char, 16)).zfill(4) for char in file_data)
    packet, _ = parse_packet(binary_data)
    return sum_versions(packet)

def resolve_packet(packet):
    if packet['type'] == 0:
        return sum(resolve_packet(subpacket) for subpacket in packet['subpackets'])
    elif packet['type'] == 1:
        return math.prod(resolve_packet(subpacket) for subpacket in packet['subpackets'])
    elif packet['type'] == 2:
        return min(resolve_packet(subpacket) for subpacket in packet['subpackets'])
    elif packet['type'] == 3:
        return max(resolve_packet(subpacket) for subpacket in packet['subpackets'])
    elif packet['type'] == 4:
        return packet['value']
    elif packet['type'] == 5:
        return int(resolve_packet(packet['subpackets'][0]) > resolve_packet(packet['subpackets'][1]))
    elif packet['type'] == 6:
        return int(resolve_packet(packet['subpackets'][0]) < resolve_packet(packet['subpackets'][1]))
    elif packet['type'] == 7:
        return int(resolve_packet(packet['subpackets'][0]) == resolve_packet(packet['subpackets'][1]))

def solve2(file_data):
    binary_data = ''.join('{0:b}'.format(int(char, 16)).zfill(4) for char in file_data)
    packet, _ = parse_packet(binary_data)
    return resolve_packet(packet)

with open(sys.argv[1]) as file:
    file_data = file.read().strip()

print('part1={}'.format(solve1(file_data)))
print('part2={}'.format(solve2(file_data)))
