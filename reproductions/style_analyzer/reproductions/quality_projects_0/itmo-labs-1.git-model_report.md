# Model report for file:///tmp/top-repos-quality-repos-jffnnmlo/itmo-labs-1.git HEAD 95de8db54e9f5b3543d04c4b86027c9149d1f0a5

### Dump

```json
{'created_at': '2021-09-01 20:14:33',
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
 'size': '17.4 kB',
 'tags': [],
 'uuid': 'fde15853-5895-46f9-b1c2-22ff7b2563e3',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-jffnnmlo/itmo-labs-1.git 95de8db54e9f5b3543d04c4b86027c9149d1f0a5

# javascript
8 rules, avg.len. 5.5
## train
PPCR: 0.603995
### report
macro
{'f1-score': 0.29259774721459847,
 'precision': 0.3101303700302308,
 'recall': 0.28065453877923235,
 'support': 4989}
micro
{'f1-score': 0.9017839246341952,
 'precision': 0.9017839246341952,
 'recall': 0.9017839246341952,
 'support': 4989}
weighted
{'f1-score': 0.8898021845793629,
 'precision': 0.8897474108295629,
 'recall': 0.9017839246341952,
 'support': 4989}
### report_full
macro
{'f1-score': 0.23695125536485306,
 'precision': 0.3101303700302308,
 'recall': 0.20422915268487157,
 'support': 8260}
micro
{'f1-score': 0.6791455958940298,
 'precision': 0.9017839246341952,
 'recall': 0.5446731234866828,
 'support': 8260}
weighted
{'f1-score': 0.5950216026079087,
 'precision': 0.72430775585877,
 'recall': 0.5446731234866828,
 'support': 8260}
## test
PPCR: 0.628008
### report
macro
{'f1-score': 0.2997791423692211,
 'precision': 0.318060791728576,
 'recall': 0.28812464713851466,
 'support': 1305}
micro
{'f1-score': 0.9226053639846743,
 'precision': 0.9226053639846743,
 'recall': 0.9226053639846743,
 'support': 1305}
weighted
{'f1-score': 0.9144056350681407,
 'precision': 0.9188414384935114,
 'recall': 0.9226053639846743,
 'support': 1305}
### report_full
macro
{'f1-score': 0.2331051491250632,
 'precision': 0.318060791728576,
 'recall': 0.2004459175789835,
 'support': 2078}
micro
{'f1-score': 0.7117942654448715,
 'precision': 0.9226053639846743,
 'recall': 0.5794032723772858,
 'support': 2078}
weighted
{'f1-score': 0.6294503355007002,
 'precision': 0.7901950105384091,
 'recall': 0.5794032723772858,
 'support': 2078}
```

## javascript
### Summary
4 rules, avg.len. 4.2

| | |
|-|-|
|Min support|140|
|Max support|991|
|Min confidence|0.9248234033584595|
|Max confidence|0.9927536249160767|

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
                     'min_samples_leaf': 110,
                     'min_samples_split': 238,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  -1.reserved = :<br>	∧ -1.roles not in {STRING}<br>	∧ +1.roles in {STRING}<br>⇒ y = ␣<br>Confidence: 0.993. Support: 207.` |
| 2 | `  -1.roles not in {STRING}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.925. Support: 991.` |
| 3 | `  -1.roles not in {STRING}<br>	∧ +1.roles not in {STRING}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = ObjectExpression<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.989. Support: 140.` |
| 4 | `  -1.roles not in {STRING}<br>	∧ +1.reserved = =<br>	∧ +1.roles not in {STRING}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.970. Support: 219.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 4.25, "max_conf": 0.9927536249160767, "max_support": 991, "min_conf": 0.9248234033584595, "min_support": 140, "num_rules": 4}}
```
</details>
