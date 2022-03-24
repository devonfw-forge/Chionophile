import os

csv_files = [f for f in os.listdir(os.path.join('..','results')) if '.csv' in f and 'PowerLog' in f]

# Cleaning csv files:
for csvf in csv_files:
    with open(os.path.join('..','results',csvf), "r", encoding='utf-8') as f:
        is_metrics = False
        lines_list = []
        for line in f:
            if 'Total Elapsed Time' in line: 
                is_metrics = True
            if is_metrics: lines_list.append(line)
                        
        clean_csv = ''.join(lines_list) 

    num = int(csvf.split('.')[0].split('_')[1])

    if num == 0: new_csv = 'power_rust'
    elif num == 1: new_csv = 'power_java'
    elif num == 2: new_csv = 'power_node'
    else: new_csv = 'power_undefined'

    count = 0
    while os.path.exists(os.path.join('..','results',new_csv+'_'+str(count)+'.csv')):
        count += 1
    new_csv += '_'+str(count)+'.csv'
    
    os.remove(os.path.join('..','results',csvf))
    with open(os.path.join('..','results',new_csv), "w", encoding='utf-8') as f:
        f.write(clean_csv)


report_files = [f for f in os.listdir(os.path.join('..','results')) if '.html' in f and 'report' in f]

# Cleaning html report files:
for report in report_files:
    with open(os.path.join('..','results',report), "r", encoding='utf-8') as f:
        is_table = False
        is_head_table = False
        is_end_table = False
        html_list = []
        for line in f:
            
            if '<table>\n' in line: 
                is_table = True
                html_list.append(line)
            
            if not is_table: html_list.append(line)

            if is_table and '<thead>' in line:
                is_head_table = True
            if is_table and is_head_table and 'Task' not in line: 
                html_list.append(
                    line.replace('colspan="2"','').replace('colspan="3"',''))
            elif is_table and is_head_table and 'Task' in line:  
                html_list.append(line)
            if is_table and is_head_table and '</thead>' in line:
                is_head_table = False

            if is_table and 'Aggregated' in line:
                html_list.append('<tbody>\n')
                html_list.append('<tr>\n')
                html_list.append(
                    line.replace('colspan="2"','').replace('colspan="3"',''))
                is_end_table = True
            if is_table and is_end_table: 
                html_list.append(
                    line.replace('colspan="2"','').replace('colspan="3"',''))
            if is_table and is_end_table and '</tr>' in line:
                is_end_table = False
                html_list.append('</tbody>\n')
                
            if '</table>' in line: 
                is_table = False
                html_list.append(line)
            
        clean_html = ''.join(html_list) 

    num = int(report.split('.')[0].split('_')[1])

    if num == 0: new_report = 'res_rust'
    elif num == 1: new_report = 'res_java'
    elif num == 2: new_report = 'res_node'
    else: new_report = 'res_undefined'

    count = 0
    while os.path.exists(os.path.join('..','results',new_report+'_'+str(count)+'.html')):
        count += 1
    new_report += '_'+str(count)+'.html'
    
    os.remove(os.path.join('..','results',report))
    with open(os.path.join('..','results',new_report), "w", encoding='utf-8') as f:
        f.write(clean_html)