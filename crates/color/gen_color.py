import csv

for row in csv.reader(open('color_swatch.csv')):
    name, code = row
    r = int(code[1:3], 16) / 255
    g = int(code[3:5], 16) / 255
    b = int(code[5:7], 16) / 255
    print(f'impl_color!({name}, {r}, {g}, {b});')
