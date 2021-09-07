# Model report for file:///tmp/top-repos-quality-repos-niet5311/qgis-webappbuilder-plugin.git HEAD ceed00caa091ca875feef624c1cf4a12bb23cda4

### Dump

```json
{'created_at': '2021-09-01 03:27:22',
 'datasets': [],
 'dependencies': [],
 'description': 'Model bound to style.format.analyzer.FormatAnalyzer Lookout analyzer.',
 'environment': {'packages': 'ConfigArgParse==0.13.0 Jinja2==2.10 MarkupSafe==1.1.1 PyStemmer==1.3.0 PyYAML==5.1 Pympler==0.5 SQLAlchemy==1.2.10 SQLAlchemy-Utils==0.33.3 asdf==2.3.2 bblfsh==2.12.7 boto==2.49.0 boto3==1.9.130 botocore==1.12.130 cachetools==2.0.1 certifi==2019.3.9 chardet==3.0.4 clint==0.5.1 docker==3.7.0 docker-pycreds==0.4.0 dulwich==0.19.11 grpcio==1.19.0 grpcio-tools==1.19.0 humanfriendly==4.16.1 humanize==0.5.1 idna==2.8 jmespath==0.9.4 jsonschema==2.6.0 lookout-sdk==0.4.1 lookout-sdk-ml==0.19.0 lookout-style==0.2.0 lz4==2.1.6 modelforge==0.12.1 numpy==1.16.2 packaging==19.0 pandas==0.22.0 pip==19.0.3 protobuf==3.7.0 psycopg2-binary==2.7.5 pygtrie==2.3 pyparsing==2.3.1 python-dateutil==2.8.0 python-igraph==0.7.1.post6 pytz==2019.1 requests==2.21.0 requirements-parser==0.2.0 scikit-learn==0.20.1 scikit-optimize==0.5.2 scipy==1.2.1 semantic-version==2.6.0 setuptools==40.8.0 six==1.12.0 smart-open==1.8.1 sourced-ml==0.8.2 spdx==2.5.0 stringcase==1.2.0 tabulate==0.8.2 tqdm==4.31.1 '
                             'urllib3==1.24.1 websocket-client==0.55.0 xxhash==1.3.0',
                 'platform': 'Linux-4.15.0-135-generic-x86_64-with-Ubuntu-18.04-bionic',
                 'python': '3.6.7 (default, Oct 22 2018, 11:32:17) [GCC 8.2.0]'},
 'license': 'ODbL-1.0',
 'metrics': {},
 'model': 'style.format.analyzer.FormatAnalyzer',
 'references': [],
 'series': 'Lookout',
 'size': '18.1 kB',
 'tags': [],
 'uuid': '7d45f474-ca46-49c8-8920-11b4b597ecee',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-niet5311/qgis-webappbuilder-plugin.git ceed00caa091ca875feef624c1cf4a12bb23cda4

# javascript
15 rules, avg.len. 5.2
## train
PPCR: 0.816554
### report
macro
{'f1-score': 0.5893412014003196,
 'precision': 0.6110375026306838,
 'recall': 0.57288728408906,
 'support': 14640}
micro
{'f1-score': 0.9355191256830601,
 'precision': 0.9355191256830601,
 'recall': 0.9355191256830601,
 'support': 14640}
weighted
{'f1-score': 0.9301424965070768,
 'precision': 0.9261850132437041,
 'recall': 0.9355191256830601,
 'support': 14640}
### report_full
macro
{'f1-score': 0.4867827334109611,
 'precision': 0.6110375026306838,
 'recall': 0.4331906981162914,
 'support': 17929}
micro
{'f1-score': 0.8410451656483159,
 'precision': 0.9355191256830601,
 'recall': 0.7639020581181326,
 'support': 17929}
weighted
{'f1-score': 0.8060420136385333,
 'precision': 0.8818646911326872,
 'recall': 0.7639020581181326,
 'support': 17929}
## test
PPCR: 0.852373
### report
macro
{'f1-score': 0.5496398311980388,
 'precision': 0.6089174580384571,
 'recall': 0.5306109834088526,
 'support': 4723}
micro
{'f1-score': 0.9665466864281177,
 'precision': 0.9665466864281177,
 'recall': 0.9665466864281177,
 'support': 4723}
weighted
{'f1-score': 0.9634385979942975,
 'precision': 0.9630290621745934,
 'recall': 0.9665466864281177,
 'support': 4723}
### report_full
macro
{'f1-score': 0.48774255891504886,
 'precision': 0.6089174580384571,
 'recall': 0.4516719746048116,
 'support': 5541}
micro
{'f1-score': 0.8895167575993764,
 'precision': 0.9665466864281177,
 'recall': 0.8238585092943512,
 'support': 5541}
weighted
{'f1-score': 0.8584898418229051,
 'precision': 0.918075178415238,
 'recall': 0.8238585092943512,
 'support': 5541}
```

## javascript
### Summary
5 rules, avg.len. 5.4

| | |
|-|-|
|Min support|162|
|Max support|5554|
|Min confidence|0.9205729365348816|
|Max confidence|0.999454140663147|

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
                     'min_samples_leaf': 95,
                     'min_samples_split': 203,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  +1.length ≥ 2<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 916.` |
| 2 | `  -1.reserved not in {(}<br>	∧ -2.diff_offset ≥ 4<br>	∧ +1.length ≥ 2<br>	∧ +2.length ≤ 2<br>	∧ ^1.roles not in {LITERAL, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.921. Support: 1920.` |
| 3 | `  -1.reserved = {<br>	∧ -2.diff_offset ≤ 3<br>	∧ +1.length ≥ 2<br>	∧ +3.roles not in {LITERAL}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.944. Support: 278.` |
| 4 | `  -1.reserved not in {(, {}<br>	∧ -2.diff_offset ≤ 3<br>	∧ -2.label not in {<space>}<br>	∧ +1.length ≥ 2<br>	∧ +2.roles in {FUNCTION}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎⏎<br>Confidence: 0.985. Support: 162.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.roles not in {FUNCTION}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.962. Support: 5554.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 5.4, "max_conf": 0.999454140663147, "max_support": 5554, "min_conf": 0.9205729365348816, "min_support": 162, "num_rules": 5}}
```
</details>
