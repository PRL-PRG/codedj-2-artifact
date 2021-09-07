# Model report for file:///tmp/top-repos-quality-repos-y_kfgbkg/niagara_falls_answers.git HEAD 31e233af8283aba82ae72a776cc166ad99181558

### Dump

```json
{'created_at': '2021-09-02 01:10:10',
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
 'size': '16.6 kB',
 'tags': [],
 'uuid': 'b4b9e7f4-289f-450e-9bcd-690e1148d2d9',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-y_kfgbkg/niagara_falls_answers.git 31e233af8283aba82ae72a776cc166ad99181558

# javascript
8 rules, avg.len. 6.4
## train
PPCR: 0.801428
### report
macro
{'f1-score': 0.44079764863135507,
 'precision': 0.43897147349663235,
 'recall': 0.44344496304781955,
 'support': 6736}
micro
{'f1-score': 0.9103325415676959,
 'precision': 0.9103325415676959,
 'recall': 0.9103325415676959,
 'support': 6736}
weighted
{'f1-score': 0.8966640742141881,
 'precision': 0.8840165439097043,
 'recall': 0.9103325415676959,
 'support': 6736}
### report_full
macro
{'f1-score': 0.3696575690735554,
 'precision': 0.43897147349663235,
 'recall': 0.327958018666404,
 'support': 8405}
micro
{'f1-score': 0.8099861303744799,
 'precision': 0.9103325415676959,
 'recall': 0.729565734681737,
 'support': 8405}
weighted
{'f1-score': 0.7726924791569585,
 'precision': 0.8347189906173351,
 'recall': 0.729565734681737,
 'support': 8405}
## test
PPCR: 0.752128
### report
macro
{'f1-score': 0.4308702563555737,
 'precision': 0.4323812783573173,
 'recall': 0.43147560281598374,
 'support': 2209}
micro
{'f1-score': 0.908103214124038,
 'precision': 0.908103214124038,
 'recall': 0.908103214124038,
 'support': 2209}
weighted
{'f1-score': 0.8926138504155032,
 'precision': 0.8781004382704142,
 'recall': 0.908103214124038,
 'support': 2209}
### report_full
macro
{'f1-score': 0.3176208386437148,
 'precision': 0.4323812783573173,
 'recall': 0.27352096139802484,
 'support': 2937}
micro
{'f1-score': 0.7796346677030703,
 'precision': 0.908103214124038,
 'recall': 0.68300987402111,
 'support': 2937}
weighted
{'f1-score': 0.7357122409408594,
 'precision': 0.8367454413756757,
 'recall': 0.68300987402111,
 'support': 2937}
```

## javascript
### Summary
2 rules, avg.len. 8.5

| | |
|-|-|
|Min support|122|
|Max support|2780|
|Min confidence|0.9559352397918701|
|Max confidence|0.9959016442298889|

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
| 1 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved = var<br>	∧ -1.roles not in {STRING}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ␣<br>Confidence: 0.996. Support: 122.` |
| 2 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {,, ;, var, {}<br>	∧ -1.roles not in {STRING}<br>	∧ -3.diff_line = 0<br>	∧ +1.reserved not in {{}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.956. Support: 2780.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 8.5, "max_conf": 0.9959016442298889, "max_support": 2780, "min_conf": 0.9559352397918701, "min_support": 122, "num_rules": 2}}
```
</details>
