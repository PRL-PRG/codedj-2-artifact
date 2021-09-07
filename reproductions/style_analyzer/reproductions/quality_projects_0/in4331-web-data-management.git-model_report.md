# Model report for file:///tmp/top-repos-quality-repos-9il6yuvw/in4331-web-data-management.git HEAD 565314e978861aea62b3d2bd6e53d48d5e4f6c20

### Dump

```json
{'created_at': '2021-09-01 17:10:24',
 'datasets': [],
 'dependencies': [],
 'description': 'Model bound to style.format.analyzer.FormatAnalyzer Lookout analyzer.',
 'environment': {'packages': 'ConfigArgParse==0.13.0 Jinja2==2.10 MarkupSafe==1.1.1 PyStemmer==1.3.0 PyYAML==5.1 Pympler==0.5 SQLAlchemy==1.2.10 SQLAlchemy-Utils==0.33.3 asdf==2.3.2 bblfsh==2.12.7 boto==2.49.0 boto3==1.9.130 botocore==1.12.130 cachetools==2.0.1 certifi==2019.3.9 chardet==3.0.4 clint==0.5.1 docker==3.7.0 docker-pycreds==0.4.0 dulwich==0.19.11 grpcio==1.19.0 grpcio-tools==1.19.0 humanfriendly==4.16.1 humanize==0.5.1 idna==2.8 jmespath==0.9.4 jsonschema==2.6.0 lookout-sdk==0.4.1 lookout-sdk-ml==0.19.0 lookout-style==0.2.0 lz4==2.1.6 modelforge==0.12.1 numpy==1.16.2 packaging==19.0 pandas==0.22.0 pip==19.0.3 protobuf==3.7.0 psycopg2-binary==2.7.5 pygtrie==2.3 pyparsing==2.3.1 python-dateutil==2.8.0 python-igraph==0.7.1.post6 pytz==2019.1 requests==2.21.0 requirements-parser==0.2.0 scikit-learn==0.20.1 scikit-optimize==0.5.2 scipy==1.2.1 semantic-version==2.6.0 setuptools==40.8.0 six==1.12.0 smart-open==1.8.1 sourced-ml==0.8.2 spdx==2.5.0 stringcase==1.2.0 tabulate==0.8.2 tqdm==4.31.1 '
                             'urllib3==1.24.1 websocket-client==0.55.0 xxhash==1.3.0',
                 'platform': 'Linux-5.4.0-74-generic-x86_64-with-Ubuntu-18.04-bionic',
                 'python': '3.6.7 (default, Oct 22 2018, 11:32:17) [GCC 8.2.0]'},
 'license': 'ODbL-1.0',
 'metrics': {},
 'model': 'style.format.analyzer.FormatAnalyzer',
 'references': [],
 'series': 'Lookout',
 'size': '18.7 kB',
 'tags': [],
 'uuid': '00336049-1720-4da2-8d3b-b0f06ba74c18',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-9il6yuvw/in4331-web-data-management.git 565314e978861aea62b3d2bd6e53d48d5e4f6c20

# javascript
15 rules, avg.len. 5.5
## train
PPCR: 0.845629
### report
macro
{'f1-score': 0.37499016972347154,
 'precision': 0.3762403653311689,
 'recall': 0.3739558310945211,
 'support': 12632}
micro
{'f1-score': 0.9538473717542749,
 'precision': 0.9538473717542748,
 'recall': 0.9538473717542748,
 'support': 12632}
weighted
{'f1-score': 0.9469232601384542,
 'precision': 0.9403440989361227,
 'recall': 0.9538473717542748,
 'support': 12632}
### report_full
macro
{'f1-score': 0.3494793995101012,
 'precision': 0.3762403653311689,
 'recall': 0.3296115066671336,
 'support': 14938}
micro
{'f1-score': 0.8740660137830976,
 'precision': 0.9538473717542748,
 'recall': 0.8066006158789664,
 'support': 14938}
weighted
{'f1-score': 0.8282917934515134,
 'precision': 0.8546248984435891,
 'recall': 0.8066006158789664,
 'support': 14938}
## test
PPCR: 0.884307
### report
macro
{'f1-score': 0.3705583518272568,
 'precision': 0.3682811468026106,
 'recall': 0.3731403816370257,
 'support': 3195}
micro
{'f1-score': 0.9489827856025039,
 'precision': 0.9489827856025039,
 'recall': 0.9489827856025039,
 'support': 3195}
weighted
{'f1-score': 0.9397249813554927,
 'precision': 0.9310453276349754,
 'recall': 0.9489827856025039,
 'support': 3195}
### report_full
macro
{'f1-score': 0.35273253613638944,
 'precision': 0.3682811468026106,
 'recall': 0.34021145438320366,
 'support': 3613}
micro
{'f1-score': 0.8907168037602821,
 'precision': 0.9489827856025039,
 'recall': 0.8391918073623028,
 'support': 3613}
weighted
{'f1-score': 0.8479388354226518,
 'precision': 0.85831015971908,
 'recall': 0.8391918073623028,
 'support': 3613}
```

## javascript
### Summary
10 rules, avg.len. 5.1

| | |
|-|-|
|Min support|100|
|Max support|2044|
|Min confidence|0.9265941381454468|
|Max confidence|0.9990137815475464|

### Configuration

```json
{'feature_extractor': {'cutoff_label_support': 80,
                       'debug_parsing': False,
                       'label_composites': '<cut>',
                       'left_features': ['length',
                                         'diff_offset',
                                         'diff_col',
                                         'diff_line',
                                         'internal_type',
                                         'label',
                                         'reserved',
                                         'roles'],
                       'left_siblings_window': 5,
                       'no_labels_on_right': True,
                       'node_features': ['start_line', 'start_col'],
                       'parent_features': ['internal_type', 'roles'],
                       'parents_depth': 2,
                       'return_sibling_indices': False,
                       'right_features': ['length', 'internal_type', 'reserved', 'roles'],
                       'right_siblings_window': 5,
                       'select_features_number': 500,
                       'selected_features': '<cut>'},
 'line_length_limit': 500,
 'lines_ratio_train_trigger': 0.2,
 'lower_bound_instances': 500,
 'optimizer': {'base_model_name_categories': ['sklearn.ensemble.RandomForestClassifier',
                                              'sklearn.tree.DecisionTreeClassifier'],
               'cv': 3,
               'max_depth_categories': [None, 5, 10],
               'max_features_categories': [None, 'auto'],
               'min_samples_leaf_max': 120,
               'min_samples_leaf_min': 90,
               'min_samples_split_max': 240,
               'min_samples_split_min': 180,
               'n_iter': 50,
               'n_jobs': -1},
 'overall_size_limit': 5242880,
 'random_state': 42,
 'test_dataset_ratio': 0.2,
 'trainable_rules': {'attribute_similarity_threshold': 0.98,
                     'base_model_name': 'sklearn.tree.DecisionTreeClassifier',
                     'confidence_threshold': 0.8,
                     'max_depth': 10,
                     'min_samples_leaf': 90,
                     'min_samples_split': 180,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.970. Support: 2044.` |
| 2 | `  +1.length ≥ 2<br>	∧ ^1.internal_type = ObjectExpression<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.939. Support: 470.` |
| 3 | `  -1.label in {<space>}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {ObjectExpression}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = "<br>Confidence: 0.938. Support: 365.` |
| 4 | `  •••start_line ≥ 17<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {ObjectExpression}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.994. Support: 236.` |
| 5 | `  •••start_col ≥ 6<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {ObjectExpression}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.927. Support: 2023.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 402.` |
| 7 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 164.` |
| 8 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 507.` |
| 9 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 1840.` |
| 10 | `  -1.diff_col ≥ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = (<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.975. Support: 100.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 5.1, "max_conf": 0.9990137815475464, "max_support": 2044, "min_conf": 0.9265941381454468, "min_support": 100, "num_rules": 10}}
```
</details>
