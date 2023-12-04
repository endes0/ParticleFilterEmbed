from math import *
import random
import numpy as np
import pylab as pl


# definición de las balizas
landmarks  = [[20.0, 20.0], [80.0, 80.0], [20.0, 80.0], [80.0, 20.0]]

# tamaño del mundo
world_size = 100.0


class robot:
    def __init__(self):
        
        # robot se inicializa con un valor aleatorio uniforme de posición y orientación, y con ruidos con desviación típica 0
        
        self.x = random.random() * world_size
        self.y = random.random() * world_size
        self.orientation = random.random() * 2.0 * pi
        self.forward_noise = 0.0;
        self.turn_noise    = 0.0;
        self.sense_noise   = 0.0;
    
    def set(self, new_x, new_y, new_orientation):
        # cambia la posición y la orientación
        if new_x < 0 or new_x >= world_size:
            raise ValueError; 'Coordenada X fuera de los límites'
        if new_y < 0 or new_y >= world_size:
            raise ValueError; 'Coordenada Y fuera de los límites'
        if new_orientation < 0 or new_orientation >= 2 * pi:
            raise ValueError; 'La orientación debe estar entre 0 y 2pi'
        self.x = float(new_x)
        self.y = float(new_y)
        self.orientation = float(new_orientation)
    
    
    def set_noise(self, new_f_noise, new_t_noise, new_s_noise):
        
        # cambia la desviación típica de los ruidos de avance, de giro y de medida
        
        self.forward_noise = float(new_f_noise);
        self.turn_noise    = float(new_t_noise);
        self.sense_noise   = float(new_s_noise);
    
    
    def sense(self):
        
        # calcula las distancias a cada una de las balizas
        
        Z = []
        for i in range(len(landmarks)):
            dist = sqrt((self.x - landmarks[i][0]) ** 2 + (self.y - landmarks[i][1]) ** 2)
            dist += random.gauss(0.0, self.sense_noise)
            Z.append(dist)
        return Z

    def numeros(self):
        
        # devuelve los datos numéricos de posición y orientación
        
        V=[]
        V.append(self.x)
        V.append(self.y)
        V.append(self.orientation)
        return V

    
    def move(self, turn, forward):

        # calcula la posición y la orientación cuando el robot se mueve

        if forward < 0:
            raise ValueError; 'El robot no se puede mover hacia atrás'         
        
        # gira y añade el ruido de giro
        orientation = self.orientation + float(turn) + random.gauss(0.0, self.turn_noise)
        orientation %= 2 * pi  # truncamiento cíclico
        
        # avanza, y añade el ruido de avance
        dist = float(forward) + random.gauss(0.0, self.forward_noise)
        x = self.x + (cos(orientation) * dist)
        y = self.y + (sin(orientation) * dist)
        x %= world_size  # truncamiento cíclico  
        y %= world_size  # truncamiento cíclico
        
        # guarda los nuevos valores
        res = robot()
        res.set(x, y, orientation)
        res.set_noise(self.forward_noise, self.turn_noise, self.sense_noise)
        return res


    def Gaussian(self, mu, sigma, x):
        
        # calcula la probabilidad de x para una gaussiana con media mu y desviación típica sigma
        
        return exp(- ((mu - x) ** 2) / (sigma ** 2) / 2.0) / sqrt(2.0 * pi * (sigma ** 2))
    
    
    def measurement_prob(self, measurement):
        
        # calcula lo probable que debería ser una medida
        
        prob = 1.0;
        for i in range(len(landmarks)):
            dist = sqrt((self.x - landmarks[i][0]) ** 2 + (self.y - landmarks[i][1]) ** 2)
            prob *= self.Gaussian(dist, self.sense_noise, measurement[i])
        return prob
    
    
    
    def __repr__(self):

        # representación de la clase robot
        
        return '[x=%.6s y=%.6s orient=%.6s]' % (str(self.x), str(self.y), str(self.orientation))





# creación del robot
myrobot = robot()

# número de partículas
N = 1000

# número de iteraciones
T = 50 

# creación de las partículas
p = []
for i in range(N):
    r = robot()
    r.set_noise(0.05, 0.05, 5.0)
    p.append(r)


# representación gráfica
# estado inicial de las partículas
V=myrobot.numeros()
xr=V[0];
yr=V[1];

x=[]
y=[]
for i in range(N):
    V=p[i].numeros()
    x.append(V[0])
    y.append(V[1])

pl.figure()
pl.xlim(0,world_size)
pl.ylim(0,world_size)
pl.plot(x, y, 'ro')
pl.plot(xr,yr,'bo')
pl.show()
 

# filtro de partículas
for t in range(T):
    
    # movimiento del robot
    myrobot = myrobot.move(0.1, 5.0)
    Z = myrobot.sense()

    # movimiento de las partículas
    p2 = []
    for i in range(N):
        p2.append(p[i].move(0.1, 5.0))
    p = p2

    # cálculo de los pesos para las partículas
    w = []
    for i in range(N):
        w.append(p[i].measurement_prob(Z))


    # modificación de los pesos de las partículas
    # for i in range(N):
    #   w[i]=w[i]*w[i]*w[i]

    # normalización de los pesos de las partículas
    suma=0
    for i in range(N):
        suma+=w[i]
    for i in range(N):
        w[i]=w[i]/suma


    # remuestreo de las particulas
    p3=[]
    lim=[]
    lim.append(w[0])
    
    for j in range(N-1):
        lim.append(lim[j]+w[j+1])

    for j in range(N):
        aleatorio=random.random()
        i=0
        while aleatorio>lim[i]:
           i=i+1
        p3.append(p[i])
        
    p=p3

    
    # representación gráfica
    V=myrobot.numeros()
    xr=V[0];
    yr=V[1];

    x=[]
    y=[]
    for i in range(N):
        V=p[i].numeros()
        x.append(V[0])
        y.append(V[1])

    pl.cla()
    #pl.figure(t)
    pl.xlim(0,world_size)
    pl.ylim(0,world_size)
    pl.plot(x, y, 'ro')
    pl.plot(xr,yr,'bo')
    #pl.show()
    pl.pause(0.1)


