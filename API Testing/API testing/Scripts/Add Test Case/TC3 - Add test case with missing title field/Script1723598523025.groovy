import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

res = WS.sendRequest(findTestObject('Login'))

def cookie = res.getHeaderFields().get('Set-Cookie').get(0)

GlobalVariable.cookie = cookie

res2 = WS.sendRequest(findTestObject('Add test case', [('cookie') : GlobalVariable.cookie, ('title') : '', ('status') : 'Actual'
            , ('priority') : ('High'), ('severity') : 'Normal', ('desc') : 'Description', ('suite') : '-1', ('pre') : 'Precondition'
            , ('post') : 'Postcondition', ('step') : '1. Do nothing', ('expected') : 'Do nothing']))

WS.verifyResponseStatusCodeInRange(res2, 400, 499)

