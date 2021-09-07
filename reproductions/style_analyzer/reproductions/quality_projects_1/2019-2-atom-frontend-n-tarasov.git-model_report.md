# Model report for file:///tmp/top-repos-quality-repos-gil9c1pi/2019-2-atom-frontend-n-tarasov.git HEAD 8fa9ccf47a3d6aea525cda63e7dfde81b5c2d673

### Dump

```json
{'created_at': '2021-09-01 23:08:36',
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
 'size': '16.5 kB',
 'tags': [],
 'uuid': '5b36b971-a281-4d37-88f9-8084299d8c3a',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-gil9c1pi/2019-2-atom-frontend-n-tarasov.git 8fa9ccf47a3d6aea525cda63e7dfde81b5c2d673

# javascript
10 rules, avg.len. 6.3
## train
PPCR: 0.654085
### report
macro
{'f1-score': 0.4493010452781493,
 'precision': 0.4411150487228728,
 'recall': 0.458910881380602,
 'support': 5476}
micro
{'f1-score': 0.9344411979547115,
 'precision': 0.9344411979547115,
 'recall': 0.9344411979547115,
 'support': 5476}
weighted
{'f1-score': 0.9218715971440087,
 'precision': 0.9107616366881689,
 'recall': 0.9344411979547115,
 'support': 5476}
### report_full
macro
{'f1-score': 0.3658135815539457,
 'precision': 0.4411150487228728,
 'recall': 0.31643200689323453,
 'support': 8372}
micro
{'f1-score': 0.7390236857307915,
 'precision': 0.9344411979547115,
 'recall': 0.6112040133779264,
 'support': 8372}
weighted
{'f1-score': 0.7001764780957274,
 'precision': 0.8282628764995986,
 'recall': 0.6112040133779264,
 'support': 8372}
## test
PPCR: 0.665479
### report
macro
{'f1-score': 0.45862518531148494,
 'precision': 0.45791349710216445,
 'recall': 0.4598061147668415,
 'support': 1494}
micro
{'f1-score': 0.9524765729585006,
 'precision': 0.9524765729585006,
 'recall': 0.9524765729585006,
 'support': 1494}
weighted
{'f1-score': 0.9472740461124542,
 'precision': 0.9430060815147936,
 'recall': 0.9524765729585006,
 'support': 1494}
### report_full
macro
{'f1-score': 0.36491240351157767,
 'precision': 0.45791349710216445,
 'recall': 0.30772350907004126,
 'support': 2245}
micro
{'f1-score': 0.7611660871890878,
 'precision': 0.9524765729585006,
 'recall': 0.6338530066815145,
 'support': 2245}
weighted
{'f1-score': 0.720098840996377,
 'precision': 0.8508609911969988,
 'recall': 0.6338530066815145,
 'support': 2245}
```

## javascript
### Summary
6 rules, avg.len. 6.0

| | |
|-|-|
|Min support|141|
|Max support|1167|
|Min confidence|0.9315476417541504|
|Max confidence|0.996962308883667|

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
| 1 | `  ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.956. Support: 1167.` |
| 2 | `  -1.diff_col ≤ 10<br>	∧ -1.diff_offset ≥ 3<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.955. Support: 478.` |
| 3 | `  -1.diff_offset ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;, {}<br>	∧ -2.label in {<space>}<br>	∧ -3.roles in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.996. Support: 141.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {VARIABLE} and not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.932. Support: 168.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {INCOMPLETE, OPERATOR, QUALIFIED, VARIABLE}<br>⇒ y = ␣<br>Confidence: 0.941. Support: 179.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles in {IDENTIFIER}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {>}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 823.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 6.0, "max_conf": 0.996962308883667, "max_support": 1167, "min_conf": 0.9315476417541504, "min_support": 141, "num_rules": 6}}
```
</details>
