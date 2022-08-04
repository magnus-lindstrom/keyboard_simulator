from collections import Counter
from modules import get_counts, freq_from_count, save_freqs

"""
This script gets the word frequency of words in the swedish and english
languages and prints it to text files. It also saves the most common bigrams
from each language and the most common letters. Together with frequencies.
"""


data_dir = '../src/stats/'
languages = ['eng', 'swe']
lang_importance = [.5, .5]

# Empty counters
tot_word_freqs = Counter()
tot_bigram_freqs = Counter()
tot_letter_freqs = Counter()

for i_lang, lang in enumerate(languages):
    word_count, bigram_count, letter_count = get_counts(lang, data_dir)

    word_freqs = freq_from_count(word_count)
    bigram_freqs = freq_from_count(bigram_count)
    letter_freqs = freq_from_count(letter_count)

    save_freqs(word_freqs, '{}_word_frequency'.format(lang), data_dir)
    save_freqs(bigram_freqs, '{}_bigram_frequency'.format(lang), data_dir)
    save_freqs(letter_freqs, '{}_letter_frequency'.format(lang), data_dir)

    # Place specified emphasis on both languages
    for word in word_freqs:
        tot_word_freqs[word] += word_freqs[word] * lang_importance[i_lang]
    for bigram in bigram_freqs:
        tot_bigram_freqs[bigram] += bigram_freqs[bigram] * lang_importance[i_lang]
    for letter in letter_freqs:
        tot_letter_freqs[letter] += letter_freqs[letter] * lang_importance[i_lang]

save_freqs(tot_word_freqs, 'tot_word_frequency', data_dir)
save_freqs(tot_bigram_freqs, 'tot_bigram_frequency', data_dir)
save_freqs(tot_letter_freqs, 'tot_letter_frequency', data_dir)

word_freq_sum = sum(tot_word_freqs.values())
bigram_freq_sum = sum(tot_bigram_freqs.values())
letter_freq_sum = sum(tot_letter_freqs.values())
if word_freq_sum < .99 or word_freq_sum > 1.01:
    print('ERROR! Total word frequencies summed to {:.02f}!'.format(word_freq_sum))
if bigram_freq_sum < .99 or bigram_freq_sum > 1.01:
    print('ERROR! Total bigram frequencies summed to {:.02f}!'.format(bigram_freq_sum))
if letter_freq_sum < .99 or letter_freq_sum > 1.01:
    print('ERROR! Total letter frequencies summed to {:.02f}!'.format(letter_freq_sum))
