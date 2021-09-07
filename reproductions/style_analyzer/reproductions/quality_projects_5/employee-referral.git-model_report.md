# Model report for file:///tmp/top-repos-quality-repos-k_dgv63y/employee-referral.git HEAD e22271e89f4ec92df5d7cdfbcfa4d52637b1eb2f

### Dump

```json
{'created_at': '2021-09-02 10:01:18',
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
 'size': '18.5 kB',
 'tags': [],
 'uuid': '1d03ac59-7b21-4880-b1e5-ab17093e82e2',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-k_dgv63y/employee-referral.git e22271e89f4ec92df5d7cdfbcfa4d52637b1eb2f

# javascript
98 rules, avg.len. 7.3
## train
PPCR: 0.987841
### report
macro
{'f1-score': 0.7280679915863103,
 'precision': 0.7983822835577552,
 'recall': 0.6923044501804261,
 'support': 92374}
micro
{'f1-score': 0.9710632862060753,
 'precision': 0.9710632862060753,
 'recall': 0.9710632862060753,
 'support': 92374}
weighted
{'f1-score': 0.9684354574295869,
 'precision': 0.9682656600136795,
 'recall': 0.9710632862060753,
 'support': 92374}
### report_full
macro
{'f1-score': 0.7067026704225512,
 'precision': 0.7983822835577552,
 'recall': 0.6665677804024542,
 'support': 93511}
micro
{'f1-score': 0.9651235979234473,
 'precision': 0.9710632862060753,
 'recall': 0.9592561302948316,
 'support': 93511}
weighted
{'f1-score': 0.9608617711107856,
 'precision': 0.966792426503849,
 'recall': 0.9592561302948316,
 'support': 93511}
## test
PPCR: 0.979470
### report
macro
{'f1-score': 0.7076267049437038,
 'precision': 0.7523092528788623,
 'recall': 0.6916190714760759,
 'support': 18511}
micro
{'f1-score': 0.9521365674463832,
 'precision': 0.9521365674463832,
 'recall': 0.9521365674463832,
 'support': 18511}
weighted
{'f1-score': 0.9483521318779548,
 'precision': 0.948015597730612,
 'recall': 0.9521365674463832,
 'support': 18511}
### report_full
macro
{'f1-score': 0.6831097682231473,
 'precision': 0.7523092528788623,
 'recall': 0.6602185386624642,
 'support': 18899}
micro
{'f1-score': 0.942261427425822,
 'precision': 0.9521365674463832,
 'recall': 0.9325890258743849,
 'support': 18899}
weighted
{'f1-score': 0.9359844357979842,
 'precision': 0.9456093662492222,
 'recall': 0.9325890258743849,
 'support': 18899}
```

## javascript
### Summary
53 rules, avg.len. 6.6

| | |
|-|-|
|Min support|139|
|Max support|42557|
|Min confidence|0.9257246255874634|
|Max confidence|0.999799370765686|

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
                     'base_model_name': 'sklearn.ensemble.RandomForestClassifier',
                     'confidence_threshold': 0.8,
                     'min_samples_leaf': 90,
                     'min_samples_split': 182,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  -1.roles in {EXPRESSION}<br>	∧ -2.reserved = =<br>	∧ ^1.internal_type = JSXOpeningElement<br>⇒ y = "<br>Confidence: 0.999. Support: 2607.` |
| 2 | `  -1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type = JSXOpeningElement<br>⇒ y = ␣<br>Confidence: 0.987. Support: 6956.` |
| 3 | `  -2.roles in {INCOMPLETE}<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.internal_type not in {JSXOpeningElement}<br>⇒ y = "<br>Confidence: 0.999. Support: 4983.` |
| 4 | `  -1.reserved = :<br>	∧ -2.label not in {<space>}<br>	∧ -2.roles not in {INCOMPLETE}<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.internal_type not in {JSXOpeningElement}<br>⇒ y = ␣<br>Confidence: 0.936. Support: 242.` |
| 5 | `  -1.internal_type = StringLiteral<br>	∧ -3.internal_type = JSXIdentifier<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type not in {JSXOpeningElement}<br>⇒ y = "<br>Confidence: 1.000. Support: 2492.` |
| 6 | `  -1.internal_type = StringLiteral<br>	∧ -3.internal_type not in {JSXIdentifier}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type not in {JSXOpeningElement}<br>⇒ y = '<br>Confidence: 0.964. Support: 857.` |
| 7 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.roles not in {EXPRESSION}<br>	∧ -4.reserved not in {=}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {/}<br>	∧ +3.length ≥ 17<br>	∧ ^1.internal_type not in {JSXOpeningElement}<br>	∧ ^1.roles in {INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 2478.` |
| 8 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.roles not in {EXPRESSION}<br>	∧ -4.reserved not in {=}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +3.length ≤ 16<br>	∧ ^1.internal_type not in {JSXOpeningElement}<br>	∧ ^1.roles in {INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 39618.` |
| 9 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type = MemberExpression<br>	∧ ^1.roles not in {INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.991. Support: 2166.` |
| 10 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {JSXOpeningElement, MemberExpression}<br>	∧ ^1.roles not in {INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.992. Support: 295.` |
| 11 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {JSXOpeningElement, MemberExpression}<br>	∧ ^1.roles not in {INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.956. Support: 394.` |
| 12 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = import<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {JSXOpeningElement, MemberExpression}<br>	∧ ^1.roles not in {INCOMPLETE}<br>⇒ y = ⏎<br>Confidence: 0.983. Support: 151.` |
| 13 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = from<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {JSXOpeningElement, MemberExpression}<br>	∧ ^1.roles not in {INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 222.` |
| 14 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {from, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {JSXOpeningElement}<br>	∧ ^1.roles not in {INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.967. Support: 2286.` |
| 15 | `  -1.roles in {EXPRESSION}<br>	∧ -3.internal_type = JSXIdentifier<br>	∧ ^1.internal_type = JSXOpeningElement<br>⇒ y = "<br>Confidence: 0.999. Support: 2540.` |
| 16 | `  -1.internal_type = StringLiteral<br>	∧ -3.roles in {INCOMPLETE}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type not in {JSXOpeningElement}<br>⇒ y = "<br>Confidence: 1.000. Support: 2476.` |
| 17 | `  -1.internal_type = StringLiteral<br>	∧ -3.roles not in {INCOMPLETE}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type not in {JSXOpeningElement}<br>⇒ y = '<br>Confidence: 0.972. Support: 880.` |
| 18 | `  -1.internal_type not in {StringLiteral}<br>	∧ -4.reserved not in {=}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type not in {JSXOpeningElement}<br>	∧ ^1.roles in {INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.991. Support: 42557.` |
| 19 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type not in {JSXOpeningElement}<br>	∧ ^1.roles in {IDENTIFIER} and not in {INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.990. Support: 2105.` |
| 20 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {JSXOpeningElement}<br>	∧ ^1.roles not in {IDENTIFIER, INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.992. Support: 325.` |
| 21 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {JSXOpeningElement}<br>	∧ ^1.roles not in {IDENTIFIER, INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.932. Support: 362.` |
| 22 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = from<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {JSXOpeningElement}<br>	∧ ^1.roles not in {IDENTIFIER, INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 216.` |
| 23 | `  -1.diff_col ≤ 5<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ><br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {JSXOpeningElement}<br>	∧ ^1.roles not in {INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 150.` |
| 24 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.roles not in {EXPRESSION}<br>	∧ -4.reserved not in {), =}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {/}<br>	∧ +3.length ≥ 17<br>	∧ ^1.internal_type not in {JSXOpeningElement}<br>	∧ ^1.roles in {INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 2408.` |
| 25 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.roles not in {EXPRESSION}<br>	∧ -4.reserved not in {), =}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +3.length ≤ 16<br>	∧ ^1.internal_type not in {JSXOpeningElement}<br>	∧ ^1.roles in {INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 39431.` |
| 26 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type not in {JSXOpeningElement}<br>	∧ ^1.roles in {QUALIFIED} and not in {INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.990. Support: 2100.` |
| 27 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {JSXOpeningElement}<br>	∧ ^1.roles not in {INCOMPLETE, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 321.` |
| 28 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {JSXOpeningElement}<br>	∧ ^1.roles not in {INCOMPLETE, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.977. Support: 377.` |
| 29 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = from<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {JSXOpeningElement}<br>	∧ ^1.roles not in {INCOMPLETE, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 182.` |
| 30 | `  -1.roles in {EXPRESSION}<br>	∧ -3.roles in {INCOMPLETE}<br>	∧ ^1.internal_type = JSXOpeningElement<br>⇒ y = "<br>Confidence: 0.998. Support: 2648.` |
| 31 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.reserved not in {=}<br>	∧ -2.roles not in {EXPRESSION}<br>	∧ -4.reserved not in {=}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {/, }}<br>	∧ +3.length ≥ 17<br>	∧ ^1.internal_type not in {JSXOpeningElement}<br>	∧ ^1.roles in {INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 2360.` |
| 32 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.reserved not in {=}<br>	∧ -2.roles not in {EXPRESSION}<br>	∧ -4.reserved not in {=}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +3.length ≤ 16<br>	∧ ^1.internal_type not in {JSXOpeningElement}<br>	∧ ^1.roles in {INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 39172.` |
| 33 | `  -1.diff_col ≤ 1<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {JSXOpeningElement, MemberExpression}<br>	∧ ^1.roles not in {INCOMPLETE}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.959. Support: 302.` |
| 34 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;, {}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {from, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {JSXOpeningElement}<br>	∧ ^1.roles not in {INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.954. Support: 2242.` |
| 35 | `  -1.roles in {EXPRESSION}<br>	∧ +1.roles in {INCOMPLETE}<br>	∧ ^1.internal_type = JSXOpeningElement<br>⇒ y = "<br>Confidence: 0.998. Support: 2617.` |
| 36 | `  -1.internal_type = StringLiteral<br>	∧ -2.reserved = =<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type not in {JSXOpeningElement}<br>⇒ y = "<br>Confidence: 1.000. Support: 2470.` |
| 37 | `  -1.internal_type = StringLiteral<br>	∧ -2.reserved not in {=}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type not in {JSXOpeningElement}<br>⇒ y = '<br>Confidence: 0.967. Support: 806.` |
| 38 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -5.length ≥ 2<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {JSXOpeningElement, MemberExpression}<br>	∧ ^1.roles not in {INCOMPLETE}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.934. Support: 173.` |
| 39 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {JSXOpeningElement, MemberExpression}<br>	∧ ^1.roles not in {INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.947. Support: 347.` |
| 40 | `  -1.diff_col ≤ 4<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ><br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {JSXOpeningElement}<br>	∧ ^1.roles not in {INCOMPLETE}<br>	∧ ^2.roles not in {MAP}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 160.` |
| 41 | `  -1.roles in {EXPRESSION}<br>	∧ +1.internal_type = JSXIdentifier<br>	∧ ^1.internal_type = JSXOpeningElement<br>⇒ y = "<br>Confidence: 0.998. Support: 2632.` |
| 42 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ,<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {JSXOpeningElement}<br>	∧ ^1.roles in {DECLARATION} and not in {IDENTIFIER, INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 257.` |
| 43 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {,}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {JSXOpeningElement}<br>	∧ ^1.roles in {DECLARATION} and not in {FUNCTION, IDENTIFIER, INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.933. Support: 694.` |
| 44 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {JSXOpeningElement}<br>	∧ ^1.roles not in {DECLARATION, IDENTIFIER, INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.996. Support: 139.` |
| 45 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {JSXOpeningElement}<br>	∧ ^1.roles not in {DECLARATION, INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.969. Support: 1893.` |
| 46 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = import<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {JSXOpeningElement}<br>	∧ ^1.roles not in {IDENTIFIER, INCOMPLETE}<br>⇒ y = ⏎<br>Confidence: 0.969. Support: 144.` |
| 47 | `  -2.internal_type = JSXIdentifier<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.internal_type not in {JSXOpeningElement}<br>⇒ y = "<br>Confidence: 0.999. Support: 5114.` |
| 48 | `  -2.internal_type not in {JSXIdentifier}<br>	∧ -2.label in {<space>}<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.internal_type not in {JSXOpeningElement}<br>⇒ y = ␣<br>Confidence: 0.926. Support: 276.` |
| 49 | `  -1.reserved = :<br>	∧ -2.internal_type not in {JSXIdentifier}<br>	∧ -2.label not in {<space>}<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.internal_type not in {JSXOpeningElement}<br>⇒ y = ␣<br>Confidence: 0.943. Support: 218.` |
| 50 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ,<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {JSXOpeningElement}<br>	∧ ^1.roles in {DECLARATION} and not in {INCOMPLETE, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 257.` |
| 51 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {,}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {JSXOpeningElement}<br>	∧ ^1.roles in {DECLARATION} and not in {FUNCTION, INCOMPLETE, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.955. Support: 659.` |
| 52 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {JSXOpeningElement}<br>	∧ ^1.roles not in {DECLARATION, INCOMPLETE, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.985. Support: 1749.` |
| 53 | `  -1.roles in {EXPRESSION}<br>	∧ +1.roles in {IDENTIFIER}<br>	∧ ^1.internal_type = JSXOpeningElement<br>⇒ y = "<br>Confidence: 0.999. Support: 2688.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 6.6415094339622645, "max_conf": 0.999799370765686, "max_support": 42557, "min_conf": 0.9257246255874634, "min_support": 139, "num_rules": 53}}
```
</details>
