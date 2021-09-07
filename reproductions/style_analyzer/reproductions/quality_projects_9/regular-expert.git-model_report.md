# Model report for file:///tmp/top-repos-quality-repos-fb5u__4v/regular-expert.git HEAD 81456c308df79e9137a6e1c4778e928c6c08c7b3

### Dump

```json
{'created_at': '2021-08-31 21:15:20',
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
 'size': '15.7 kB',
 'tags': [],
 'uuid': 'b482c884-0c05-431f-995c-2b8e1335a314',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-fb5u__4v/regular-expert.git 81456c308df79e9137a6e1c4778e928c6c08c7b3

# javascript
11 rules, avg.len. 6.9
## train
PPCR: 0.737147
### report
macro
{'f1-score': 0.5325780487899642,
 'precision': 0.5461483094333152,
 'recall': 0.5234001849834689,
 'support': 6366}
micro
{'f1-score': 0.9359095193213949,
 'precision': 0.9359095193213949,
 'recall': 0.9359095193213949,
 'support': 6366}
weighted
{'f1-score': 0.9201503929177983,
 'precision': 0.9065173670952945,
 'recall': 0.9359095193213949,
 'support': 6366}
### report_full
macro
{'f1-score': 0.4474160414979754,
 'precision': 0.5461483094333152,
 'recall': 0.3898975135621914,
 'support': 8636}
micro
{'f1-score': 0.7942940941207839,
 'precision': 0.9359095193213949,
 'recall': 0.6899027327466419,
 'support': 8636}
weighted
{'f1-score': 0.7488774856302453,
 'precision': 0.8392236844069626,
 'recall': 0.6899027327466419,
 'support': 8636}
## test
PPCR: 0.728912
### report
macro
{'f1-score': 0.5085498945167227,
 'precision': 0.5309809263593449,
 'recall': 0.49555122358793074,
 'support': 1374}
micro
{'f1-score': 0.9221251819505094,
 'precision': 0.9221251819505094,
 'recall': 0.9221251819505094,
 'support': 1374}
weighted
{'f1-score': 0.9075668238145298,
 'precision': 0.8959290087826839,
 'recall': 0.9221251819505094,
 'support': 1374}
### report_full
macro
{'f1-score': 0.42923064162084046,
 'precision': 0.5309809263593449,
 'recall': 0.37389676147719314,
 'support': 1885}
micro
{'f1-score': 0.7775391224301933,
 'precision': 0.9221251819505094,
 'recall': 0.6721485411140583,
 'support': 1885}
weighted
{'f1-score': 0.7302994331941993,
 'precision': 0.8154101725860908,
 'recall': 0.6721485411140583,
 'support': 1885}
```

## javascript
### Summary
6 rules, avg.len. 5.3

| | |
|-|-|
|Min support|95|
|Max support|2828|
|Min confidence|0.9424242377281189|
|Max confidence|0.9979919791221619|

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
| 1 | `  +1.reserved = ><br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 95.` |
| 2 | `  -1.reserved not in {{}<br>	∧ -1.roles not in {FUNCTION}<br>	∧ -4.diff_line = 0<br>	∧ -5.label not in {<space>}<br>	∧ +1.reserved not in {>}<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.953. Support: 546.` |
| 3 | `  -1.internal_type = StringLiteral<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = "<br>Confidence: 0.998. Support: 249.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.roles in {MAP}<br>	∧ +1.roles in {MAP}<br>	∧ ^1.internal_type not in {ObjectExpression}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.942. Support: 165.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {MAP}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.995. Support: 102.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -2.diff_offset ≥ 3<br>	∧ -2.roles not in {MAP}<br>	∧ -3.reserved not in {;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {MAP}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.internal_type not in {IfStatement}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.949. Support: 2828.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 5.333333333333333, "max_conf": 0.9979919791221619, "max_support": 2828, "min_conf": 0.9424242377281189, "min_support": 95, "num_rules": 6}}
```
</details>
