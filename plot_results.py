import matplotlib.pyplot as plt

# open results file
with open('results.txt', 'r') as f:
    # go through each line
    for line in f:
        # trim
        line = line.strip()
        # check if line starts with 'I'
        if line.startswith('I'):
            plt.pause(0.0001)
            plt.cla()
            plt.xlim(0, 100)
            plt.ylim(0, 100)
        # check if line starts with 'P'
        elif line.startswith('P'):
            # the line by ;
            line = line.split(';')
            # get the x and y values
            x = float(line[1])
            y = float(line[2])
            # plot
            plt.plot(x, y, 'ro')
        # check if line starts with 'R'
        elif line.startswith('R'):
            # the line by ;
            line = line.split(';')
            # get the x and y values
            x = float(line[1])
            y = float(line[2])
            # plot
            plt.plot(x, y, 'bo')
