#!/usr/bin/python3
'''[summary]

Script to run the TTC binary
'''
import argparse
import subprocess
import math
import json
import pandas as pd
from matplotlib.pyplot import *

ROOT = f"."
DATA = f"{ROOT}/data"
IMAGE = f"{ROOT}/image"
JSON = f"{ROOT}/json"
TARGET = f"{ROOT}/target/release"


def plot_ttc_curve(program, thread_cnt):
	data_file = f"{JSON}/{program}-t{thread_cnt}-ttc.json"
	f = open(data_file)
	data = json.load(f)
	cache_size = list(map(lambda t: t[0], data))
	ttc = list(map(lambda t: t[-1], data))
	fig = figure(figsize=(12,8))
	ax = fig.add_subplot(2,1,1)
	plot(cache_size, ttc, label="TTC", linestyle="-", linewidth=2, color="#44546A", alpha=1.0)
	# ax.set_title(f"{program}", fontsize=18, fontweight='bold')
	ax.set_xlabel('Cache Size', fontsize=14, fontweight='bold')
	ax.set_ylabel('TTC', fontsize=14, fontweight='bold')
	grid(True)
	legend()

	clb = list(map(lambda t: t[1], data))
	cub = list(map(lambda t: t[2], data))
	ax = fig.add_subplot(2,1,2)
	axes = list()
	axes.append(plot(cache_size, clb, label="Denminator", linestyle="-", linewidth=2, color="#44546A", alpha=1.0))
	axes.append(plot(cache_size, cub, label="Nominator", linestyle=":", linewidth=4, color="#C8A593", alpha=1.0))
	# ax.set_title(f"{program}", fontsize=18, fontweight='bold')
	ax.set_xlabel('Cache Size', fontsize=14, fontweight='bold')
	ax.set_ylabel('Nominator and Denominator to compute TTC', fontsize=14, fontweight='bold')
	grid(True)
	fig.suptitle(program, fontsize=18)
	legend()
	savefig(f"{IMAGE}/{program}-t{thread_cnt}-ttc.png")


if __name__ == '__main__':
	parser = argparse.ArgumentParser()
	parser.add_argument('-p', '--prog', action='store', dest='benchmarks',
					type=str, nargs='*',
					help='set the program to be run, if not set, run all benchmarks')
	parser.add_argument('-t', '--tgroup', action='store', dest='thread_cnts',
					type=int, nargs='*', default=[2,4,6,8,10],
					help='set the set of threds to be run, if not set, run the default [2,4,6,8,10] setting')
	parser.add_argument('-c', '--cache', type=int, default=512,
					help='set the cache size to check')

	args = parser.parse_args()
	for program in args.benchmarks:
		for t in args.thread_cnts:
			cmd = f"TTC_LOG=INFO {TARGET}/ttc --input {DATA}/{program}-t{t}-pin-rih-0.data -m {args.cache} -o {JSON}/{program}-t{t}-ttc.json "
			print(f"run {cmd} ...")
			proc = subprocess.Popen(cmd, shell=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
			stdout, stderr = proc.communicate()
			plot_ttc_curve(program, t)
