#!/usr/bin/python3
'''[summary]

Script to run the TTC binary
'''
import argparse
import subprocess
import math
import json
import os
import re
import copy
from matplotlib.pyplot import *

ROOT = f"."
DATA = f"{ROOT}/data"
IMAGE = f"{ROOT}/image"
JSON = f"{ROOT}/json"
TARGET = f"{ROOT}/target/release"


def write_to_file(fname, content):
	f =open(fname, "w")
	f.write(content)
	f.close()



def read_histogram(name):
	histogram = dict()
	total_ri_cnt = 0.0
	with open(name) as f:
		for line in f:
			line = line.rstrip()
			m = re.search(rf"^(.*?),(.*?),(.*?)$", line)
			if not m:
				continue
			ri = int(m.group(1))
			ri_freq = float(m.group(2))
			if ri in histogram:
				histogram[ri] += ri_freq
			else:
				histogram[ri] = ri_freq
			total_ri_cnt += ri_freq 
	# convert histogram to distribution
	large_ris = 0.0
	for ri in histogram:
		histogram[ri] = histogram[ri] / total_ri_cnt
		if ri >= 1024:
			large_ris += histogram[ri]
	print(f"large ri occupies {large_ris}")
	return histogram

def convert_histogram_to_log2(name):
	histogram = dict()
	total_ri_cnt = 0.0
	with open(name) as f:
		for line in f:
			line = line.rstrip()
			m = re.search(rf"^(.*?),(.*?),(.*?)$", line)
			if not m:
				continue
			ri = int(m.group(1))
			ri_freq = float(m.group(2))
			log2_ri = pow(2, int(math.log2(ri)))
			if log2_ri in histogram:
				histogram[log2_ri] += ri_freq
			else:
				histogram[log2_ri] = ri_freq
			total_ri_cnt += ri_freq
	content = ["Start to dump reuse time"] 
	for ri in histogram:
		content.append(f"{ri},{histogram[ri]},{histogram[ri] / total_ri_cnt}")
		histogram[ri] = histogram[ri] / total_ri_cnt
	write_to_file(name, "\n".join(content))


# figure_setting struct
# {
# 	row,
# 	col,
# 	i: col_id,
# 	prev_axes,
# }
def plot_ttc_figure_entry(fig, fig_config, json_data, histogram, ttc_label):
	# plot TTC line
	ax1 = fig.add_subplot(fig_config[0],fig_config[1],fig_config[2])
	print(f"{fig_config[0]},{fig_config[1]},{fig_config[2]}")
	cache_size = list(map(lambda t: t[0]*64/1024, json_data))
	ttc = list(map(lambda t: t[-2], json_data))
	ax1.plot(cache_size, ttc, label=ttc_label, linestyle="-", linewidth=2, color="#44546A", alpha=1.0)
	ax1.set_xlabel('Cache Size (KB)', fontsize=14, fontweight='bold')
	ax1.set_ylabel('TTC', fontsize=14, fontweight='bold')
	grid(True)
	legend()
	# plot the ce/cs line
	clb = list(map(lambda t: t[1]*64/1024, json_data))
	cub = list(map(lambda t: t[2]*64/1024, json_data))
	ax2 = fig.add_subplot(fig_config[0],fig_config[1],fig_config[2]+fig_config[1])
	print(f"{fig_config[0]},{fig_config[1]},{fig_config[2]+fig_config[1]}")
	ax2.plot(cache_size, clb, label="Cs", linestyle="-", linewidth=2, color="#44546A", alpha=1.0)
	ax2.plot(cache_size, cub, label="Ce", linestyle=":", linewidth=4, color="#C8A593", alpha=1.0)
	ax2.set_xlabel('Cache Size (KB)', fontsize=14, fontweight='bold')
	ax2.set_ylabel('Ce/Cs', fontsize=14, fontweight='bold')
	grid(True)
	legend()
	# plot the histogram
	ri = sorted(histogram.keys())
	ri_freq = [histogram[i] for i in ri]
	if fig_config[-1]:
		ax3 = fig.add_subplot(fig_config[0],fig_config[1],fig_config[2]+fig_config[1]*2, sharey=fig_config[-1][2])
	else:
		ax3 = fig.add_subplot(fig_config[0],fig_config[1],fig_config[2]+fig_config[1]*2)
	print(f"{fig_config[0]},{fig_config[1]},{fig_config[2]+fig_config[1]*2}")
	ax3.scatter(ri, ri_freq, s=16, color="#44546A", alpha=1.0)
	ax3.set_xlabel('RI', fontsize=14, fontweight='bold')
	ax3.set_ylabel('Distribution', fontsize=14, fontweight='bold')
	ax3.set_xscale('log', base=2)
	grid(True)
	# plot the miss ratio
	miss_ratio = list(map(lambda t: t[-1], json_data))
	if fig_config[-1]:
		ax4 = fig.add_subplot(fig_config[0],fig_config[1],fig_config[2]+fig_config[1]*3, sharey=fig_config[-1][3])
	else:
		ax4 = fig.add_subplot(fig_config[0],fig_config[1],fig_config[2]+fig_config[1]*3)
	
	print(f"{fig_config[0]},{fig_config[1]},{fig_config[2]+fig_config[1]*3}")
	ax4.plot(cache_size, miss_ratio, linestyle="-", linewidth=4, color="#C8A593", alpha=1.0)
	ax4.set_xlabel('Cache Size (KB)', fontsize=14, fontweight='bold')
	ax4.set_ylabel('Miss Ratio', fontsize=14, fontweight='bold')
	grid(True)
	return [ax1, ax2, ax3, ax4]



def plot_ttc_curve(figure_name, title, orig_data, orig_histogram, tiled_data=None, tiled_histogram=None):
	if orig_data and tiled_data:
		print(f"PLOT ORIG and TILE TTC")
	elif orig_data:
		print(f"PLOT ORIG TTC")
	elif tiled_data:
		print(f"PLOT TILE TTC")
	fig = figure(figsize=(12,8))
	col = 1 if not tiled_data else 2
	figure_cnt = 4*col
	row = figure_cnt // col
	figure_idx = 1
	prev_axes = None
	if orig_data:
		prev_axes = plot_ttc_figure_entry(fig, (row, col, figure_idx, prev_axes), orig_data, orig_histogram, "TTC (ORIG)")
		figure_idx += 1
	if tiled_data:
		prev_axes = plot_ttc_figure_entry(fig, (row, col, figure_idx, prev_axes), tiled_data, tiled_histogram, "TTC (TILED)")
		figure_idx += 1
	fig.suptitle(title, fontsize=18)
	savefig(f"{IMAGE}/{figure_name}")

def intropolate_ttc(json_data):
	for i in range(len(json_data)):
		json_entry = json_data[i]
		cache_size = json_entry[0]
		ce = json_entry[2]
		cs = json_entry[1]
		real_ttc = cache_size // cs
		print(f"Before: {json_entry[-2]} ({ce} // {cs})")
		print(f"After: {real_ttc} ({cache_size} // {cs})")
		json_data[i][-2] = real_ttc





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
			orig_data, tiled_data = None, None
			orig_histogram, tiled_histogram = dict(), dict()
			orig_data_file, tiled_data_file = None, None
			# find unoptimized dataset
			if os.path.exists(f"{DATA}/orig/{program}-t{t}-pluss-pro-model-ri-rih.data"):
				orig_data_file = f"{DATA}/orig/{program}-t{t}-pluss-pro-model-ri-rih.data"
			elif os.path.exists(f"{DATA}/orig/{program}-t{t}-c4-pin-rih-0.data"):
				convert_histogram_to_log2(f"{DATA}/orig/{program}-t{t}-c4-pin-rih-0.data")
				orig_data_file = f"{DATA}/orig/{program}-t{t}-c4-pin-rih-0.data"
			# find tiled dataset
			if os.path.exists(f"{DATA}/tiled/{program}-t{t}-c4-pin-rih-0.data"):
				convert_histogram_to_log2(f"{DATA}/tiled/{program}-t{t}-c4-pin-rih-0.data")
				tiled_data_file = f"{DATA}/tiled/{program}-t{t}-c4-pin-rih-0.data"
			elif os.path.exists(f"{DATA}/tiled/{program}-t{t}-pluss-pro-model-ri-rih.data"):
				tiled_data_file = f"{DATA}/tiled/{program}-t{t}-pluss-pro-model-ri-rih.data"
			# if os.path.exists(f"{DATA}/orig/{program}-t{t}-c4-pin-rih-0.data"):
			# 	convert_histogram_to_log2(f"{DATA}/orig/{program}-t{t}-c4-pin-rih-0.data")
			# 	tiled_data_file = f"{DATA}/orig/{program}-t{t}-c4-pin-rih-0.data"
			if orig_data_file:
				cmd = f"TTC_LOG=INFO {TARGET}/ttc unshared --input {orig_data_file} -m {args.cache} -o {JSON}/{program}-t{t}-ttc-orig.json "
				print(f"run {cmd} ...")
				proc = subprocess.Popen(cmd, shell=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
				stdout, stderr = proc.communicate()
				data_file = f"{JSON}/{program}-t{t}-ttc-orig.json"
				f = open(data_file)
				orig_data = json.load(f)
				orig_histogram = read_histogram(orig_data_file)
				tiled_data = copy.deepcopy(orig_data)
				intropolate_ttc(tiled_data)
				tiled_histogram = read_histogram(orig_data_file)
				f.close()


			'''
			if tiled_data_file:
				cmd = f"TTC_LOG=INFO {TARGET}/ttc unshared --input {tiled_data_file} -m {args.cache} -o {JSON}/{program}-t{t}-ttc-tiled.json "
				print(f"run {cmd} ...")
				proc = subprocess.Popen(cmd, shell=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
				stdout, stderr = proc.communicate()
				data_file = f"{JSON}/{program}-t{t}-ttc-tiled.json"
				f = open(data_file)
				tiled_data = intropolate_ttc(json.load(f))
				tiled_histogram = read_histogram(tiled_data_file)
				f.close()
			'''
			if not orig_data and not tiled_data:
				print(f"SKIP {program}, No histogram found")
				continue
			plot_ttc_curve(f"{program}-t{t}-ttc.png", f"{program}", orig_data, orig_histogram, tiled_data=tiled_data, tiled_histogram=tiled_histogram)
