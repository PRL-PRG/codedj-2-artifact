# Model report for file:///tmp/top-repos-quality-repos-j1zadpqs/certificates-for-everyone-netlify.git HEAD 04db893536e992b028c2f4cfe2e79bfbcd3e9b1b

### Dump

```json
{'created_at': '2021-09-02 04:35:11',
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
 'size': '15.9 kB',
 'tags': [],
 'uuid': '9433ef91-81ba-458c-b48b-e289b635465a',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-j1zadpqs/certificates-for-everyone-netlify.git 04db893536e992b028c2f4cfe2e79bfbcd3e9b1b

# javascript
11 rules, avg.len. 6.3
## train
PPCR: 0.759371
### report
macro
{'f1-score': 0.6361368823534592,
 'precision': 0.6526132144902128,
 'recall': 0.6257497592292077,
 'support': 7678}
micro
{'f1-score': 0.9286272466788226,
 'precision': 0.9286272466788226,
 'recall': 0.9286272466788226,
 'support': 7678}
weighted
{'f1-score': 0.9172410219162003,
 'precision': 0.907953450425197,
 'recall': 0.9286272466788226,
 'support': 7678}
### report_full
macro
{'f1-score': 0.5032299200388258,
 'precision': 0.6526132144902128,
 'recall': 0.45152281426973845,
 'support': 10111}
micro
{'f1-score': 0.8016189780201247,
 'precision': 0.9286272466788226,
 'recall': 0.7051725843141133,
 'support': 10111}
weighted
{'f1-score': 0.7635256798849042,
 'precision': 0.8769957690757312,
 'recall': 0.7051725843141133,
 'support': 10111}
## test
PPCR: 0.744214
### report
macro
{'f1-score': 0.65308291794427,
 'precision': 0.6618975899410815,
 'recall': 0.6515308753106012,
 'support': 1254}
micro
{'f1-score': 0.9306220095693781,
 'precision': 0.930622009569378,
 'recall': 0.930622009569378,
 'support': 1254}
weighted
{'f1-score': 0.9221955524877845,
 'precision': 0.9182744200844487,
 'recall': 0.930622009569378,
 'support': 1254}
### report_full
macro
{'f1-score': 0.5138521482782973,
 'precision': 0.6618975899410815,
 'recall': 0.4766012035211321,
 'support': 1685}
micro
{'f1-score': 0.7941476692752637,
 'precision': 0.930622009569378,
 'recall': 0.6925816023738872,
 'support': 1685}
weighted
{'f1-score': 0.7562636025113975,
 'precision': 0.8922446501692353,
 'recall': 0.6925816023738872,
 'support': 1685}
```

## javascript
### Summary
6 rules, avg.len. 7.5

| | |
|-|-|
|Min support|90|
|Max support|3904|
|Min confidence|0.9322580695152283|
|Max confidence|0.9987593293190002|

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
| 1 | `  -1.internal_type = StringLiteral<br>⇒ y = "<br>Confidence: 0.999. Support: 403.` |
| 2 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.label in {<space>}<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = ␣<br>Confidence: 0.967. Support: 137.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = import<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ⏎<br>Confidence: 0.961. Support: 90.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles not in {DECLARATION}<br>	∧ ^2.internal_type = File<br>⇒ y = ␣<br>Confidence: 0.932. Support: 155.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;}<br>	∧ -1.roles not in {COMMENT}<br>	∧ -2.roles not in {LITERAL}<br>	∧ -4.diff_offset ≥ 5<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {COMMENT} and not in {MAP}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles in {FILE} and not in {DECLARATION, IF, OPERATOR}<br>	∧ ^2.internal_type not in {File}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 122.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;}<br>	∧ -1.roles not in {COMMENT}<br>	∧ -2.roles not in {LITERAL}<br>	∧ -4.diff_offset ≥ 5<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {MAP}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles not in {DECLARATION, FILE, IF, OPERATOR}<br>	∧ ^2.internal_type not in {File}<br>⇒ y = ∅<br>Confidence: 0.943. Support: 3904.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 7.5, "max_conf": 0.9987593293190002, "max_support": 3904, "min_conf": 0.9322580695152283, "min_support": 90, "num_rules": 6}}
```
</details>
