import csv

raw_data_path = 'raw_data.csv'
four_week_average_path = 'four_week_average.csv'





# read raw_data.csv and calculate the four week average for every 5 minute
def calculateFourWeekAverage():
    with open(raw_data_path, mode='r') as file:
        reader = csv.reader(file)
        data = list(reader)     
        data = data[1:]
        data = sorted(data, key=lambda x: x[1])
        data = [x[0] for x in data]
        data = [int(x) for x in data]
        data = [data[i:i+4] for i in range(0, len(data), 4)]
        data = [sum(x)/4 for x in data]
        with open(four_week_average_path, mode='w') as file:
            writer = csv.writer(file)
            writer.writerow(data)

