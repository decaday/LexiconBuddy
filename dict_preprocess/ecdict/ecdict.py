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
        # 0 word	单词名称
        # 1 phonetic	音标，以英语英标为主
        # 2 definition	单词释义（英文），每行一个释义
        # 3 translation	单词释义（中文），每行一个释义
        # 4 pos	词语位置，用 "/" 分割不同位置
        # 5 collins	柯林斯星级
        # 6 oxford	是否是牛津三千核心词汇
        # 7 tag	字符串标签：zk/中考，gk/高考，cet4/四级 等等标签，空格分割
        # 8 bnc	英国国家语料库词频顺序
        # 9 frq	当代语料库词频顺序
        # 10 exchange	时态复数等变换，使用 "/" 分割不同项目，见后面表格
        # 11 detail	json 扩展信息，字典形式保存例句（待添加）
        # 12 audio 读音音频 url （待添加）
        dic_data = [row[0], row[1], row[2], row[3], row[4], row[10], row[5], row[7], row[9]]
        #            0       1         2     3        4        5         6        7    8
        #           word    phonetic def    tran     pos    exchange    collins  tag    frq

        data_str = ""
        for data_row in dic_data:
            data_str += (data_row + ",")
        # get addr of word str
        tell = dic_file.tell()
        # dic.append(row[0],tell)
        index_file.write(row[0] + "," + str(tell) + "\n")
        dic_file.write(data_str + "\n")

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
