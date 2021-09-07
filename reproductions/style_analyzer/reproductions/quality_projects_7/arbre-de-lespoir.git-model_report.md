# Model report for file:///tmp/top-repos-quality-repos-rjx2w1ll/arbre-de-lespoir.git HEAD 9d2569e5c368888bb727c9c88b2186ed34fc4300

### Dump

```json
{'created_at': '2021-09-01 02:28:12',
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
 'size': '13.6 kB',
 'tags': [],
 'uuid': '2d396f2d-dc36-472f-91eb-4be4b104ef2d',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-rjx2w1ll/arbre-de-lespoir.git 9d2569e5c368888bb727c9c88b2186ed34fc4300

# javascript
3 rules, avg.len. 3.3
## train
PPCR: 0.624318
### report
macro
{'f1-score': 0.37665808082792596,
 'precision': 0.37268554874036974,
 'recall': 0.38071625344352616,
 'support': 2403}
micro
{'f1-score': 0.9646275488972118,
 'precision': 0.9646275488972118,
 'recall': 0.9646275488972118,
 'support': 2403}
weighted
{'f1-score': 0.954492035790067,
 'precision': 0.9445673466316675,
 'recall': 0.9646275488972118,
 'support': 2403}
### report_full
macro
{'f1-score': 0.3052749407862354,
 'precision': 0.37268554874036974,
 'recall': 0.26523661321887987,
 'support': 3849}
micro
{'f1-score': 0.7415227127319257,
 'precision': 0.9646275488972118,
 'recall': 0.6022343465835281,
 'support': 3849}
weighted
{'f1-score': 0.6622360714096638,
 'precision': 0.7504990403181617,
 'recall': 0.6022343465835281,
 'support': 3849}
## test
PPCR: 0.184028
### report
macro
{'f1-score': 0.3324563501554652,
 'precision': 0.2997321086873326,
 'recall': 0.3768944099378882,
 'support': 106}
micro
{'f1-score': 0.7169811320754716,
 'precision': 0.7169811320754716,
 'recall': 0.7169811320754716,
 'support': 106}
weighted
{'f1-score': 0.6260080417702726,
 'precision': 0.5598928434749331,
 'recall': 0.7169811320754716,
 'support': 106}
### report_full
macro
{'f1-score': 0.11275975824122901,
 'precision': 0.2997321086873326,
 'recall': 0.0755121800085202,
 'support': 576}
micro
{'f1-score': 0.22287390029325516,
 'precision': 0.7169811320754716,
 'recall': 0.13194444444444445,
 'support': 576}
weighted
{'f1-score': 0.2069841255508199,
 'precision': 0.681962782242633,
 'recall': 0.13194444444444445,
 'support': 576}
```

## javascript
### Summary
2 rules, avg.len. 3.0

| | |
|-|-|
|Min support|331|
|Max support|1310|
|Min confidence|0.9682779312133789|
|Max confidence|0.982824444770813|

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
                     'min_samples_split': 240,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  -1.roles not in {LITERAL}<br>	∧ ^1.internal_type = JSXElement<br>⇒ y = ∅<br>Confidence: 0.983. Support: 1310.` |
| 2 | `  -1.roles not in {STRING}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles in {INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.968. Support: 331.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 3.0, "max_conf": 0.982824444770813, "max_support": 1310, "min_conf": 0.9682779312133789, "min_support": 331, "num_rules": 2}}
```
</details>
