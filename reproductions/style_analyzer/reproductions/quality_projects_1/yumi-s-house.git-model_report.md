# Model report for file:///tmp/top-repos-quality-repos-n68j2a10/yumi-s-house.git HEAD ed27ccb20ee1d10d35ed0264d74445b43188343c

### Dump

```json
{'created_at': '2021-09-02 02:50:02',
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
 'size': '15.1 kB',
 'tags': [],
 'uuid': '99073968-a0d8-4e63-8d32-51327d9fd5d4',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-n68j2a10/yumi-s-house.git ed27ccb20ee1d10d35ed0264d74445b43188343c

# javascript
5 rules, avg.len. 4.2
## train
PPCR: 0.278671
### report
macro
{'f1-score': 0.4225518284622019,
 'precision': 0.41950678576515354,
 'recall': 0.4257897874031393,
 'support': 1334}
micro
{'f1-score': 0.9775112443778111,
 'precision': 0.9775112443778111,
 'recall': 0.9775112443778111,
 'support': 1334}
weighted
{'f1-score': 0.97168949582674,
 'precision': 0.9662704433284,
 'recall': 0.9775112443778111,
 'support': 1334}
### report_full
macro
{'f1-score': 0.2282590006291993,
 'precision': 0.41950678576515354,
 'recall': 0.1589699670369902,
 'support': 4787}
micro
{'f1-score': 0.4260741708871099,
 'precision': 0.9775112443778111,
 'recall': 0.27240442866095677,
 'support': 4787}
weighted
{'f1-score': 0.4072448808674521,
 'precision': 0.8256694597338428,
 'recall': 0.27240442866095677,
 'support': 4787}
## test
PPCR: 0.267689
### report
macro
{'f1-score': 0.42857142857142855,
 'precision': 0.42857142857142855,
 'recall': 0.42857142857142855,
 'support': 227}
micro
{'f1-score': 1.0, 'precision': 1.0, 'recall': 1.0, 'support': 227}
weighted
{'f1-score': 1.0, 'precision': 1.0, 'recall': 1.0, 'support': 227}
### report_full
macro
{'f1-score': 0.2091878606494301,
 'precision': 0.42857142857142855,
 'recall': 0.14367777748125216,
 'support': 848}
micro
{'f1-score': 0.4223255813953488,
 'precision': 1.0,
 'recall': 0.267688679245283,
 'support': 848}
weighted
{'f1-score': 0.4008466170650589,
 'precision': 0.8632075471698113,
 'recall': 0.267688679245283,
 'support': 848}
```

## javascript
### Summary
4 rules, avg.len. 3.5

| | |
|-|-|
|Min support|95|
|Max support|561|
|Min confidence|0.9848484992980957|
|Max confidence|0.9970760345458984|

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
| 1 | `  -1.internal_type = StringLiteral<br>⇒ y = '<br>Confidence: 0.997. Support: 171.` |
| 2 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 143.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles in {IDENTIFIER} and not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.985. Support: 561.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = const<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {DECLARATION, IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.995. Support: 95.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 3.5, "max_conf": 0.9970760345458984, "max_support": 561, "min_conf": 0.9848484992980957, "min_support": 95, "num_rules": 4}}
```
</details>
