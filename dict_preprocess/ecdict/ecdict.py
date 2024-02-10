import csv
import os
import json

dic_file = open('output/dic.txt', 'w', encoding='utf-8')
index_file = open('output/unsort_index.txt', 'w', encoding='utf-8')
with open('dict_data/ecdict.csv', 'r', encoding="utf-8") as f:
    reader = csv.reader(f)
    ii = 0
    too_short_del_count = 0
    no_anscii_del_count = 0
    too_long_del_count = 0
    reserve_count = 0

    for row in reader:

        if len(row[0]) < 2:
            too_short_del_count += 1
            continue
        if len(row[0]) >= 30:
            too_long_del_count += 1
            continue

        del_flag = False
        for ba in row[0]:
            if ord(ba) > 128:
                del_flag = True
        if del_flag:
            no_anscii_del_count += 1
            continue

        reserve_count += 1

        dicdata = [row[0], row[1], row[3], row[7]]

        datastr = ""
        for data1 in dicdata:
            datastr += (data1.replace("\\n", "#") + ",")
        tell = dic_file.tell()
        # dic.append(row[0],tell)
        index_file.write(row[0] + "," + str(tell) + "\n")
        dic_file.write(datastr + "\n")

    print(f"too short:{too_short_del_count}, too long:{too_long_del_count}, no anscii:{no_anscii_del_count}")
    print(f"reserve:{reserve_count}")
    dic_file.close()
    index_file.close()

# sort
with open('output/unsort_index.txt', 'r') as f_in, open('output/index.txt', 'w') as f_out:
    lines = f_in.readlines()
    sorted_lines = sorted(lines, key=lambda line: [ord(char) for char in line.split(',')[0]])
    for line in sorted_lines:
        f_out.write(line)
