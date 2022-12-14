#import Constants as keys
from telegram.ext import*
from pythonapi import api_key
import telebot
bot = telebot(api_key,parse_mode=None)
bot.polling()


