# Model report for file:///tmp/top-repos-quality-repos-b3hsmgrz/reach.git HEAD 036fde6c2a83f5b014b17b423c72d79730e43a8c

### Dump

```json
{'created_at': '2021-08-31 22:59:30',
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
 'size': '15.5 kB',
 'tags': [],
 'uuid': '01089220-5bd3-4325-aa58-cce69819bfa9',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-b3hsmgrz/reach.git 036fde6c2a83f5b014b17b423c72d79730e43a8c

# javascript
8 rules, avg.len. 4.4
## train
PPCR: 0.760647
### report
macro
{'f1-score': 0.37730713072385164,
 'precision': 0.39999454355939346,
 'recall': 0.3665667410339174,
 'support': 3572}
micro
{'f1-score': 0.9053751399776035,
 'precision': 0.9053751399776035,
 'recall': 0.9053751399776035,
 'support': 3572}
weighted
{'f1-score': 0.8856164975611844,
 'precision': 0.8723455048533828,
 'recall': 0.9053751399776035,
 'support': 3572}
### report_full
macro
{'f1-score': 0.3368854712910573,
 'precision': 0.39999454355939346,
 'recall': 0.3040096129827755,
 'support': 4696}
micro
{'f1-score': 0.7822931785195936,
 'precision': 0.9053751399776035,
 'recall': 0.688671209540034,
 'support': 4696}
weighted
{'f1-score': 0.7222584289145899,
 'precision': 0.7690915607577525,
 'recall': 0.688671209540034,
 'support': 4696}
## test
PPCR: 0.774905
### report
macro
{'f1-score': 0.38467587514760204,
 'precision': 0.4039006160404076,
 'recall': 0.37377688325931463,
 'support': 1019}
micro
{'f1-score': 0.9165848871442591,
 'precision': 0.9165848871442591,
 'recall': 0.9165848871442591,
 'support': 1019}
weighted
{'f1-score': 0.9041598350212268,
 'precision': 0.897746013962788,
 'recall': 0.9165848871442591,
 'support': 1019}
### report_full
macro
{'f1-score': 0.3444489349780933,
 'precision': 0.4039006160404076,
 'recall': 0.31178218206251,
 'support': 1315}
micro
{'f1-score': 0.8003427592116539,
 'precision': 0.9165848871442591,
 'recall': 0.7102661596958175,
 'support': 1315}
weighted
{'f1-score': 0.7469826392717261,
 'precision': 0.7998523536766654,
 'recall': 0.7102661596958175,
 'support': 1315}
```

## javascript
### Summary
6 rules, avg.len. 4.2

| | |
|-|-|
|Min support|98|
|Max support|865|
|Min confidence|0.9336734414100647|
|Max confidence|0.9975961446762085|

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
| 1 | `  ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.936. Support: 480.` |
| 2 | `  +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 208.` |
| 3 | `  -1.internal_type = StringLiteral<br>	∧ +1.reserved not in {=}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.995. Support: 98.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = (<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type = CallExpression<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.988. Support: 129.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = (<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.934. Support: 98.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {(, =, {, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.935. Support: 865.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 4.166666666666667, "max_conf": 0.9975961446762085, "max_support": 865, "min_conf": 0.9336734414100647, "min_support": 98, "num_rules": 6}}
```
</details>
