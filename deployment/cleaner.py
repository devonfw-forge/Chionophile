import os

def give_name_by_number(id):
    if id == 0: return 'rust'
    elif id == 1: return 'java'
    elif id == 2: return 'node'
    elif id == 3: return 'python'
    elif id == 4: return 'net'
    else: return 'undefined'

def gen_cons_report(lines_l):
    consumption_table = "<style>\n.container2 {\nwidth: 1000px;\nmargin: 0 auto;\npadding: 10px;\nbackground: #173529;\n"
    consumption_table += "font-family: Arial, Helvetica, sans-serif;\nfont-size: 14px;\ncolor: #fff;\n}\n.info span{\n"
    consumption_table += "color: #b3c3bc;\n}\ntable {\nborder-collapse: collapse;\ntext-align: center;\nwidth: 100%;\n}\ntd, th {\n"
    consumption_table += "border: 1px solid #cad9ea;\ncolor: #666;\nheight: 30px;\n}\nthead th {\n"
    consumption_table += "background-color: #cce8eb;\nwidth: 100px;\n}\ntr:nth-child(odd) {\nbackground: #fff;\n}\n"
    consumption_table += "tr:nth-child(even) {\nbackground: #f5fafa;\n}\n.charts-container .chart {\nwidth: 100%;\n"
    consumption_table += "height: 350px;\nmargin-bottom: 30px;\n}\n.download {\nfloat: right;\n}\n.download a {\n"
    consumption_table += "color: #00ca5a;\n}\n.graph {\nmargin-bottom: 1em;\n}\n</style>\n<div class=\"container2\">\n"
    consumption_table += "<h2>Power Consumption Metrics</h2>\n<table class=\"default\">\n"
    for l in lines_l:
        consumption_table += "<tr>\n"
        consumption_table += "<td>"+l[0]+"</td>\n"
        consumption_table += "<td>"+l[1]+"</td>\n"
        consumption_table += "</tr>\n"
    consumption_table += "</table>\n"
    return consumption_table

"""
    Processing power_idle.csv file
"""
if os.path.exists(os.path.join('..','results','power_idle.csv')):
    with open(os.path.join('..','results','power_idle.csv'), "r", encoding='utf-8') as f:
        is_metrics = False
        lines_list = []
        for line in f:
            if 'Total Elapsed Time' in line: 
                is_metrics = True
            if is_metrics and len(line.split(' = ')) > 1\
                and ('Time' in line or 'Frequency' in line\
                or 'Processor' in line): 
                lines_list.append( (line.split(' = ')[0], line.split(' = ')[1]) )
        try: os.remove(os.path.join('..','results','power_idle.csv'))
        except: print("Error removing idle file")
        with open(os.path.join('..','results','power_idle.html'), "w", encoding='utf-8') as f:
            f.write(gen_cons_report(lines_list))

"""
    Processing power consumption files
"""
csv_files = [f for f in os.listdir(os.path.join('..','results')) if '.csv' in f and 'PowerLog' in f]
cons_reports = {
    'rust': {},
    'java': {},
    'node': {},
    'python': {},
    'net': {},
    'undefined': {},
}
# reading csv files:
for csvf in csv_files:
    with open(os.path.join('..','results',csvf), "r", encoding='utf-8') as f:
        is_metrics = False
        lines_list = []
        for line in f:
            if 'Total Elapsed Time' in line: 
                is_metrics = True
            if is_metrics and len(line.split(' = ')) > 1\
                and ('Time' in line or 'Frequency' in line\
                or 'Processor' in line): 
                lines_list.append( (line.split(' = ')[0], line.split(' = ')[1]) )
        # RUST
        if '_0' in csvf and 'B1' in csvf:          
            cons_reports['rust']['B1'] = gen_cons_report(lines_list)
        elif '_0' in csvf and 'B2' in csvf:          
            cons_reports['rust']['B2'] = gen_cons_report(lines_list)
        # JAVA:
        elif '_1' in csvf and 'B1' in csvf:          
            cons_reports['java']['B1'] = gen_cons_report(lines_list)
        elif '_1' in csvf and 'B2' in csvf:          
            cons_reports['java']['B2'] = gen_cons_report(lines_list)
        # NODE:
        elif '_2' in csvf and 'B1' in csvf:          
            cons_reports['node']['B1'] = gen_cons_report(lines_list)
        elif '_2' in csvf and 'B2' in csvf:          
            cons_reports['node']['B2'] = gen_cons_report(lines_list)
        # PYTHON:
        elif '_3' in csvf and 'B1' in csvf:          
            cons_reports['python']['B1'] = gen_cons_report(lines_list)
        elif '_3' in csvf and 'B2' in csvf:          
            cons_reports['python']['B2'] = gen_cons_report(lines_list)
        # NET:
        elif '_4' in csvf and 'B1' in csvf:          
            cons_reports['net']['B1'] = gen_cons_report(lines_list)
        elif '_4' in csvf and 'B2' in csvf:          
            cons_reports['net']['B2'] = gen_cons_report(lines_list)
    os.remove(os.path.join('..','results',csvf))


"""
    Processing report html files
"""
report_files = [f for f in os.listdir(os.path.join('..','results')) if '.html' in f and 'report' in f]

# Cleaning html report files:
for report in report_files:
    num = int(report.split('.')[0].split('_')[1])
    version = report.split('.')[0].split('_')[0].replace('report','')

    with open(os.path.join('..','results',report), "r", encoding='utf-8') as f:
        is_table = False; is_head_table = False; is_end_table = False; is_div_tasks = False
        html_list = []
        for line in f:
            if 'Goose Attack Report' in line:
                new_tittle = give_name_by_number(num).upper() + ' ' + version
                line = line.replace('Goose Attack Report', new_tittle)
            if '</body>' in line :
                # RUST
                if '_0' in report and 'B1' in report:          
                    html_list.append(cons_reports['rust']['B1'])
                elif '_0' in report and 'B2' in report:          
                    html_list.append(cons_reports['rust']['B2'])
                # JAVA:
                elif '_1' in report and 'B1' in report:          
                    html_list.append(cons_reports['java']['B1'])
                elif '_1' in report and 'B2' in report:          
                    html_list.append(cons_reports['java']['B2'])
                # NODE:
                elif '_2' in report and 'B1' in report:          
                    html_list.append(cons_reports['node']['B1'])
                elif '_2' in report and 'B2' in report:          
                    html_list.append(cons_reports['node']['B2'])
                # PYTHON:
                elif '_3' in report and 'B1' in report:          
                    html_list.append(cons_reports['python']['B1'])
                elif '_3' in report and 'B2' in report:          
                    html_list.append(cons_reports['python']['B2'])
                # NET:
                elif '_4' in report and 'B1' in report:          
                    html_list.append(cons_reports['net']['B1'])
                elif '_4' in report and 'B2' in report:          
                    html_list.append(cons_reports['net']['B2'])
            if '<div class=\"tasks\">' in line: is_div_tasks = True
            if '<div class=\"users\">' in line: is_div_tasks = False
            if '<table>\n' in line: 
                is_table = True; html_list.append(line)
            if not is_table and not is_div_tasks: html_list.append(line)
            if is_table and '<thead>' in line: is_head_table = True
            if is_table and is_head_table and 'Task' not in line: 
                html_list.append(
                    line.replace('colspan="2"','').replace('colspan="3"',''))
            elif is_table and is_head_table and 'Task' in line:  
                html_list.append(line)
            if is_table and is_head_table and '</thead>' in line:
                is_head_table = False
            if is_table and 'Aggregated' in line:
                html_list.append('<tbody>\n'); html_list.append('<tr>\n')
                html_list.append(
                    line.replace('colspan="2"','').replace('colspan="3"',''))
                is_end_table = True
            if is_table and is_end_table: 
                html_list.append(
                    line.replace('colspan="2"','').replace('colspan="3"',''))
            if is_table and is_end_table and '</tr>' in line:
                is_end_table = False; html_list.append('</tbody>\n')
            if '</table>' in line: 
                is_table = False; html_list.append(line)
            
        clean_html = ''.join(html_list) 

    new_report = 'res_' + give_name_by_number(num)
    count = 0
    while os.path.exists(os.path.join('..','results',new_report+version+'_'+str(count)+'.html')):
        count += 1
    new_report += version+'_'+str(count)+'.html'
    
    os.remove(os.path.join('..','results',report))
    with open(os.path.join('..','results',new_report), "w", encoding='utf-8') as f:
        f.write(clean_html)
