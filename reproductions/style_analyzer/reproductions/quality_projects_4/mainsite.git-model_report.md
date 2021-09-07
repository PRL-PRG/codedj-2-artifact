# Model report for file:///tmp/top-repos-quality-repos-1qodi0vh/mainsite.git HEAD dd9498d5b6fe3d52df9009a1e55691d0544da1fa

### Dump

```json
{'created_at': '2021-09-02 08:33:43',
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
 'size': '16.9 kB',
 'tags': [],
 'uuid': '54ea5d8b-24f3-4e1e-aaa9-bf08dbe0c0b3',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-1qodi0vh/mainsite.git dd9498d5b6fe3d52df9009a1e55691d0544da1fa

# javascript
20 rules, avg.len. 6.4
## train
PPCR: 0.910916
### report
macro
{'f1-score': 0.5863780529727656,
 'precision': 0.5921132724048161,
 'recall': 0.5811548921266432,
 'support': 22506}
micro
{'f1-score': 0.9416155691815515,
 'precision': 0.9416155691815515,
 'recall': 0.9416155691815515,
 'support': 22506}
weighted
{'f1-score': 0.9363590891073783,
 'precision': 0.9315618559028277,
 'recall': 0.9416155691815515,
 'support': 22506}
### report_full
macro
{'f1-score': 0.5473364872520388,
 'precision': 0.5921132724048161,
 'recall': 0.5126649569760021,
 'support': 24707}
micro
{'f1-score': 0.8977188486222015,
 'precision': 0.9416155691815515,
 'recall': 0.8577326263811875,
 'support': 24707}
weighted
{'f1-score': 0.8743401713352117,
 'precision': 0.8943088575499654,
 'recall': 0.8577326263811875,
 'support': 24707}
## test
PPCR: 0.902569
### report
macro
{'f1-score': 0.5960039144428487,
 'precision': 0.6006200933913297,
 'recall': 0.5919392239758736,
 'support': 5410}
micro
{'f1-score': 0.9414048059149722,
 'precision': 0.9414048059149722,
 'recall': 0.9414048059149722,
 'support': 5410}
weighted
{'f1-score': 0.934902066983307,
 'precision': 0.928974556279628,
 'recall': 0.9414048059149722,
 'support': 5410}
### report_full
macro
{'f1-score': 0.5557687652864397,
 'precision': 0.6006200933913297,
 'recall': 0.5216082034744532,
 'support': 5994}
micro
{'f1-score': 0.8931953700455981,
 'precision': 0.9414048059149722,
 'recall': 0.849683016349683,
 'support': 5994}
weighted
{'f1-score': 0.8656268686025127,
 'precision': 0.8850124755516775,
 'recall': 0.849683016349683,
 'support': 5994}
```

## javascript
### Summary
13 rules, avg.len. 6.0

| | |
|-|-|
|Min support|106|
|Max support|4603|
|Min confidence|0.9486014246940613|
|Max confidence|0.9991582632064819|

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
| 1 | `  -1.internal_type not in {StringLiteral}<br>	∧ -4.reserved not in {=}<br>	∧ ^1.internal_type = JSXElement<br>⇒ y = ∅<br>Confidence: 0.979. Support: 4603.` |
| 2 | `  -1.label in {<space>}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>⇒ y = '<br>Confidence: 0.999. Support: 594.` |
| 3 | `  -1.label not in {<space>}<br>	∧ -1.reserved = ;<br>	∧ +1.reserved = import<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {EXPRESSION}<br>⇒ y = ⏎<br>Confidence: 0.962. Support: 299.` |
| 4 | `  -1.label not in {<space>}<br>	∧ -1.reserved = =<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = JSXAttribute<br>	∧ ^1.roles not in {EXPRESSION}<br>⇒ y = "<br>Confidence: 0.998. Support: 231.` |
| 5 | `  -1.internal_type = StringLiteral<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {;}<br>	∧ -3.diff_offset ≥ 6<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXAttribute, JSXElement}<br>	∧ ^1.roles not in {EXPRESSION}<br>⇒ y = "<br>Confidence: 0.995. Support: 106.` |
| 6 | `  -1.internal_type = StringLiteral<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {JSXElement}<br>⇒ y = '<br>Confidence: 0.996. Support: 674.` |
| 7 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = )<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 136.` |
| 8 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ><br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 110.` |
| 9 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = import<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 228.` |
| 10 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 171.` |
| 11 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, import}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles in {OPERATOR} and not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.967. Support: 138.` |
| 12 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, import}<br>	∧ -5.diff_line ≤ 1<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ +2.roles in {INCOMPLETE}<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 153.` |
| 13 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, import}<br>	∧ -5.diff_line ≤ 1<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.949. Support: 4290.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 6.0, "max_conf": 0.9991582632064819, "max_support": 4603, "min_conf": 0.9486014246940613, "min_support": 106, "num_rules": 13}}
```
</details>
